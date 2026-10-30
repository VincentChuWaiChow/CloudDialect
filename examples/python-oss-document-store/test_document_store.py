"""Development tests only: these are not the migration qualification oracle."""

import base64
import contextlib
import hashlib
import io
import json
import socket
import unittest
from types import SimpleNamespace
from unittest.mock import patch

from document_store import DocumentStore, IntegrityError
import local_demo


class Download:
    def __init__(self, body, headers, *, length=None, fragment=2, error=None):
        self.body = io.BytesIO(body)
        self.headers = headers
        self.content_length = len(body) if length is None else length
        self.status = 200
        self.fragment = fragment
        self.error = error
        self.closed = False
        self.read_sizes = []

    def read(self, size):
        self.read_sizes.append(size)
        if self.error:
            raise self.error
        return self.body.read(min(size, self.fragment))

    def close(self):
        self.closed = True
        self.body.close()


class RecordingBucket:
    def __init__(self):
        self.objects = {}
        self.calls = []
        self.errors = {}
        self.statuses = {"put": 200, "delete": 204}
        self.download = None
        self.last_download = None

    def record(self, operation, key):
        self.calls.append((operation, key))
        if operation in self.errors:
            raise self.errors[operation]

    def put_object(self, key, data, headers):
        self.record("put", key)
        self.objects[key] = (data, dict(headers))
        return SimpleNamespace(status=self.statuses["put"], etag="not-an-md5")

    def get_object(self, key):
        self.record("get", key)
        if self.download is not None:
            result = self.download
        else:
            if key not in self.objects:
                raise FileNotFoundError(key)
            body, headers = self.objects[key]
            result = Download(body, {k.upper(): v for k, v in headers.items()})
        self.last_download = result
        return result

    def delete_object(self, key):
        self.record("delete", key)
        self.objects.pop(key, None)
        return SimpleNamespace(status=self.statuses["delete"])


class DocumentStoreTests(unittest.TestCase):
    def setUp(self):
        self.bucket = RecordingBucket()
        self.store = DocumentStore(self.bucket, collection="invoices", max_bytes=16)

    def save(self, body=b"invoice\x00\xff", **kwargs):
        return self.store.save("doc-01", body, filename="facture-é.pdf",
                               content_type="application/pdf", **kwargs)

    def test_upload_has_stable_key_bytes_and_explicit_metadata(self):
        document = self.save()
        self.assertEqual(self.bucket.calls, [("put", "documents/invoices/doc-01")])
        body, headers = self.bucket.objects["documents/invoices/doc-01"]
        self.assertEqual(body, b"invoice\x00\xff")
        self.assertEqual(headers, {
            "Content-Type": "application/pdf",
            "x-oss-meta-filename-b64": base64.b64encode("facture-é.pdf".encode()).decode(),
            "x-oss-meta-sha256": hashlib.sha256(body).hexdigest(),
        })
        self.assertEqual(document.body, body)
        self.assertEqual(document.sha256, hashlib.sha256(body).hexdigest())

    def test_download_preserves_unicode_metadata_and_binary_body_on_short_reads(self):
        expected = self.save()
        self.assertEqual(self.store.load("doc-01"), expected)
        self.assertTrue(self.bucket.last_download.closed)
        self.assertGreater(len(self.bucket.last_download.read_sizes), 2)

    def test_replace_updates_body_and_metadata(self):
        self.save()
        latest = self.store.save("doc-01", b"new", filename="new.txt", content_type="text/plain")
        self.assertEqual(self.store.load("doc-01"), latest)
        self.assertEqual(len(self.bucket.objects), 1)

    def test_delete_is_scoped_and_repeatable_for_the_local_contract(self):
        self.save()
        other = DocumentStore(self.bucket, collection="receipts")
        other.save("doc-01", b"keep", filename="keep.txt")
        self.store.delete("doc-01")
        self.store.delete("doc-01")
        with self.assertRaises(FileNotFoundError):
            self.store.load("doc-01")
        self.assertEqual(other.load("doc-01").body, b"keep")

    def test_empty_body_and_exact_size_limit_are_supported(self):
        for body in (b"", b"a" * 16):
            with self.subTest(body=body):
                self.save(body)
                self.assertEqual(self.store.load("doc-01").body, body)

    def test_invalid_ids_do_not_reach_any_bucket_operation(self):
        for bad in ("", ".", "..", "../x", "a/b", "a\\b", "a\n", "é", "x" * 65, None):
            for operation in (lambda: self.store.save(bad, b"x", filename="x"),
                              lambda: self.store.load(bad), lambda: self.store.delete(bad)):
                with self.subTest(bad=bad), self.assertRaises(ValueError):
                    operation()
        self.assertEqual(self.bucket.calls, [])

    def test_invalid_configuration_fails_before_io(self):
        for collection in ("", "../x", "a/b", None):
            with self.subTest(collection=collection), self.assertRaises(ValueError):
                DocumentStore(self.bucket, collection=collection)
        for limit in (0, -1, True, 1.5, "16"):
            with self.subTest(limit=limit), self.assertRaises(ValueError):
                DocumentStore(self.bucket, collection="valid", max_bytes=limit)

    def test_invalid_upload_data_or_metadata_never_calls_the_sdk(self):
        for body in ("text", bytearray(b"x"), b"x" * 17, None):
            with self.subTest(body=body), self.assertRaises(ValueError):
                self.save(body)
        for filename in ("", ".", "..", "../x", "a/b", "a\\b", "x\r\n", "\x00", "é" * 128, None):
            with self.subTest(filename=filename), self.assertRaises(ValueError):
                self.store.save("doc", b"x", filename=filename)
        for content_type in ("", "text", "text/plain\r\nInjected: yes", "text/plain; charset=utf-8", None):
            with self.subTest(content_type=content_type), self.assertRaises(ValueError):
                self.store.save("doc", b"x", filename="x", content_type=content_type)
        self.assertEqual(self.bucket.calls, [])

    def test_corrupt_body_is_rejected_even_if_etag_looks_plausible(self):
        self.save()
        _, headers = self.bucket.objects["documents/invoices/doc-01"]
        self.bucket.download = Download(b"corrupted", {**headers, "ETag": "0" * 32})
        with self.assertRaises(IntegrityError):
            self.store.load("doc-01")
        self.assertTrue(self.bucket.last_download.closed)

    def test_bad_or_missing_metadata_is_not_silently_defaulted(self):
        self.save()
        body, headers = self.bucket.objects["documents/invoices/doc-01"]
        variants = [{k: v for k, v in headers.items() if k != missing} for missing in headers]
        variants += [{**headers, "x-oss-meta-filename-b64": value}
                     for value in ("%%%", "/w==", "", "A" * 344, base64.b64encode(b"../x").decode())]
        variants += [{**headers, "x-oss-meta-sha256": "not-a-digest"},
                     {**headers, "Content-Type": "text/plain\n"}]
        for metadata in variants:
            self.bucket.download = Download(body, metadata)
            with self.subTest(metadata=metadata), self.assertRaises(IntegrityError):
                self.store.load("doc-01")
            self.assertTrue(self.bucket.last_download.closed)
            self.assertEqual(self.bucket.last_download.read_sizes, [])

    def test_advertised_oversize_and_truncated_download_are_rejected(self):
        self.save()
        body, headers = self.bucket.objects["documents/invoices/doc-01"]
        for length in (17, len(body) + 1, -1, True, "9"):
            self.bucket.download = Download(body, headers, length=length)
            with self.subTest(length=length), self.assertRaises(IntegrityError):
                self.store.load("doc-01")
            self.assertTrue(self.bucket.last_download.closed)

    def test_missing_length_is_allowed_but_read_limit_is_enforced(self):
        self.save()
        body, headers = self.bucket.objects["documents/invoices/doc-01"]
        self.bucket.download = Download(body, headers)
        self.bucket.download.content_length = None
        self.assertEqual(self.store.load("doc-01").body, body)
        self.bucket.download = Download(b"x" * 40, headers, fragment=100)
        self.bucket.download.content_length = None
        with self.assertRaises(IntegrityError):
            self.store.load("doc-01")
        self.assertEqual(self.bucket.last_download.read_sizes, [17])
        self.assertTrue(self.bucket.last_download.closed)

    def test_read_failure_closes_stream_and_is_not_retried(self):
        self.save()
        body, headers = self.bucket.objects["documents/invoices/doc-01"]
        error = OSError("synthetic transport failure")
        self.bucket.download = Download(body, headers, error=error)
        with self.assertRaises(OSError) as raised:
            self.store.load("doc-01")
        self.assertIs(raised.exception, error)
        self.assertTrue(self.bucket.last_download.closed)
        self.assertEqual(self.bucket.calls.count(("get", "documents/invoices/doc-01")), 1)

    def test_large_unknown_length_stops_at_limit_plus_one_across_chunks(self):
        self.save()
        _, headers = self.bucket.objects["documents/invoices/doc-01"]
        store = DocumentStore(self.bucket, collection="invoices", max_bytes=70000)
        self.bucket.download = Download(b"x" * 80000, headers, fragment=100000)
        self.bucket.download.content_length = None
        with self.assertRaises(IntegrityError):
            store.load("doc-01")
        self.assertEqual(self.bucket.last_download.read_sizes, [65536, 4465])
        self.assertTrue(self.bucket.last_download.closed)

    def test_provider_exceptions_propagate_without_retry(self):
        for name, operation in (("put", self.save), ("get", lambda: self.store.load("doc-01")),
                                ("delete", lambda: self.store.delete("doc-01"))):
            self.bucket.calls.clear()
            error = PermissionError("synthetic denial")
            self.bucket.errors = {name: error}
            with self.subTest(operation=name), self.assertRaises(PermissionError) as raised:
                operation()
            self.assertIs(raised.exception, error)
            self.assertEqual(self.bucket.calls, [(name, "documents/invoices/doc-01")])

    def test_unexpected_sdk_status_is_not_success(self):
        self.bucket.statuses["put"] = 202
        with self.assertRaises(RuntimeError):
            self.save()
        self.bucket.statuses["delete"] = 200
        with self.assertRaises(RuntimeError):
            self.store.delete("doc-01")
        self.bucket.download = Download(b"", {})
        self.bucket.download.status = 206
        with self.assertRaises(IntegrityError):
            self.store.load("doc-01")
        self.assertTrue(self.bucket.last_download.closed)

    def test_demo_runs_without_network_and_labels_local_evidence(self):
        output = io.StringIO()
        with patch.object(socket, "socket", side_effect=AssertionError("network forbidden")), \
             patch.object(socket, "create_connection", side_effect=AssertionError("network forbidden")), \
             contextlib.redirect_stdout(output):
            local_demo.main()
        report = json.loads(output.getvalue())
        self.assertEqual(report["evidence"], "local-only; not OSS/S3 qualification")
        self.assertEqual(report["operations"], ["put_object", "get_object", "delete_object"])
        self.assertEqual(report["checks"], {"round_trip": True, "corruption_rejected": True, "deleted": True})


if __name__ == "__main__":
    unittest.main()
