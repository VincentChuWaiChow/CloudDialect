"""Runnable in-memory demonstration. No cloud client or credentials are used."""

import io
import json
from types import SimpleNamespace

from document_store import DocumentStore, IntegrityError


class _Download(io.BytesIO):
    def __init__(self, body, headers):
        super().__init__(body)
        self.headers = dict(headers)
        self.content_length = len(body)
        self.status = 200


class MemoryBucket:
    """Only the three used API shapes; not an OSS emulator or service oracle."""

    def __init__(self):
        self.objects = {}

    def put_object(self, key, data, headers):
        self.objects[key] = (data, dict(headers))
        return SimpleNamespace(status=200)

    def get_object(self, key):
        if key not in self.objects:
            raise FileNotFoundError("Document not found in local demo")
        return _Download(*self.objects[key])

    def delete_object(self, key):
        self.objects.pop(key, None)
        return SimpleNamespace(status=204)


def main():
    bucket = MemoryBucket()
    store = DocumentStore(bucket, collection="invoices")
    saved = store.save("invoice-001", b"Invoice 001\nAmount: 125.00\n",
                       filename="facture-é.txt", content_type="text/plain")
    loaded = store.load("invoice-001")
    key = "documents/invoices/invoice-001"
    _, headers = bucket.objects[key]
    bucket.objects[key] = (b"corrupted", headers)
    corruption_rejected = False
    try:
        store.load("invoice-001")
    except IntegrityError:
        corruption_rejected = True
    store.delete("invoice-001")
    deleted = False
    try:
        store.load("invoice-001")
    except FileNotFoundError:
        deleted = True
    checks = {"round_trip": saved == loaded,
              "corruption_rejected": corruption_rejected, "deleted": deleted}
    print(json.dumps({
        "evidence": "local-only; not OSS/S3 qualification",
        "operations": ["put_object", "get_object", "delete_object"],
        "document": {"id": loaded.document_id, "filename": loaded.filename,
                     "content_type": loaded.content_type, "bytes": len(loaded.body),
                     "sha256": loaded.sha256},
        "checks": checks,
    }, ensure_ascii=False, indent=2))
    if not all(checks.values()):
        raise RuntimeError("Local demonstration failed")


if __name__ == "__main__":
    main()
