"""Bounded document storage using an explicitly supplied legacy oss2 Bucket.

No client construction, credential discovery, retries or application imports.
Collection prefixes organize keys; they are not an authorization boundary.
"""

from __future__ import annotations

import base64
from dataclasses import dataclass
import hashlib
import re
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    import oss2


class IntegrityError(ValueError):
    """The stored object cannot be returned as an intact document."""


@dataclass(frozen=True)
class Document:
    document_id: str
    filename: str
    content_type: str
    body: bytes

    @property
    def sha256(self) -> str:
        return hashlib.sha256(self.body).hexdigest()


def _segment(value: str) -> str:
    if not isinstance(value, str) or re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_-]{0,63}", value) is None:
        raise ValueError("Collection and document ID must be 1-64 ASCII letters, digits, '_' or '-'")
    return value


def _metadata(filename: str, content_type: str) -> None:
    if (not isinstance(filename, str) or not filename or not filename.isprintable()
            or filename in (".", "..") or "/" in filename or "\\" in filename
            or len(filename.encode("utf-8")) > 255):
        raise ValueError("Filename must be printable, path-free and at most 255 UTF-8 bytes")
    if (not isinstance(content_type, str) or len(content_type) > 127
            or re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9!#$&^_.+-]*/[A-Za-z0-9][A-Za-z0-9!#$&^_.+-]*", content_type) is None):
        raise ValueError("Content type must be a MIME type without parameters")


class DocumentStore:
    def __init__(self, bucket: oss2.Bucket, *, collection: str, max_bytes: int = 1024 * 1024):
        self.collection = _segment(collection)
        if type(max_bytes) is not int or max_bytes <= 0:
            raise ValueError("max_bytes must be a positive integer")
        self.bucket = bucket
        self.max_bytes = max_bytes

    def _key(self, document_id: str) -> str:
        return f"documents/{self.collection}/{_segment(document_id)}"

    def save(self, document_id: str, body: bytes, *, filename: str,
             content_type: str = "application/octet-stream") -> Document:
        """Replace one document atomically at the object level; no retry on error."""
        key = self._key(document_id)
        _metadata(filename, content_type)
        if not isinstance(body, bytes) or len(body) > self.max_bytes:
            raise ValueError("Document body must be bytes within max_bytes")
        document = Document(document_id, filename, content_type, body)
        result = self.bucket.put_object(key, body, headers={
            "Content-Type": content_type,
            "x-oss-meta-filename-b64": base64.b64encode(filename.encode("utf-8")).decode("ascii"),
            "x-oss-meta-sha256": document.sha256,
        })
        if result.status != 200:
            raise RuntimeError("Upload did not return the expected status; outcome may be unknown")
        return document

    def load(self, document_id: str) -> Document:
        """Return a bounded, metadata-validated document; always close its stream."""
        result = self.bucket.get_object(self._key(document_id))
        try:
            if result.status != 200:
                raise IntegrityError("Expected a complete object response")
            headers = {name.lower(): value for name, value in result.headers.items()}
            try:
                encoded_name = headers["x-oss-meta-filename-b64"]
                if len(encoded_name) > 340:
                    raise ValueError("Encoded filename exceeds bound")
                filename = base64.b64decode(encoded_name, validate=True).decode("utf-8")
                content_type = headers["content-type"]
                _metadata(filename, content_type)
                digest = headers["x-oss-meta-sha256"]
                if re.fullmatch(r"[0-9a-f]{64}", digest) is None:
                    raise ValueError("Invalid digest")
            except (KeyError, ValueError, TypeError) as error:
                raise IntegrityError("Invalid or missing document metadata") from error

            length = result.content_length
            if length is not None and (type(length) is not int or not 0 <= length <= self.max_bytes):
                raise IntegrityError("Invalid or excessive advertised document length")
            body = bytearray()
            while True:
                chunk = result.read(min(65536, self.max_bytes + 1 - len(body)))
                if not chunk:
                    break
                body.extend(chunk)
                if len(body) > self.max_bytes:
                    raise IntegrityError("Downloaded document exceeds max_bytes")
            if length is not None and len(body) != length:
                raise IntegrityError("Document length does not match the response")
            document = Document(document_id, filename, content_type, bytes(body))
            if document.sha256 != digest:
                raise IntegrityError("Document SHA-256 mismatch")
            return document
        finally:
            result.close()

    def delete(self, document_id: str) -> None:
        """Delete this key only; preserve provider errors without automatic retry."""
        result = self.bucket.delete_object(self._key(document_id))
        if result.status != 204:
            raise RuntimeError("Deletion did not return the expected status; outcome may be unknown")
