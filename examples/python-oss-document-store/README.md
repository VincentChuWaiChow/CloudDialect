# Python OSS document store — reference workload candidate

A purpose-built document application for CloudDialect's OSS-to-S3 migration
work. It stores named invoice/document bytes, retrieves their metadata and
integrity-checked content, replaces existing documents and deletes individual
documents. This is more than three isolated SDK examples: all three operations
share stable identity, metadata, size and failure rules.

**The runnable demo is local-only.** No migration recipe, S3 backend, cloud
client, credentials, independent oracle or qualification verdict is provided.
Creation was explicitly authorized by the user on 2026-10-30. These are generated
development examples/tests, not a held-out evaluation corpus. The repository's
existing license applies unchanged; qualification/license review and an
owner-admitted reference revision remain outstanding. No upstream gate is closed by this app.

## Run it from the repository root

```sh
rtk proxy python3 -B examples/python-oss-document-store/local_demo.py
rtk proxy python3 -B -m unittest discover -s examples/python-oss-document-store -p 'test_*.py' -v
```

No install is required. The demo prints deterministic JSON with:

```json
{
  "evidence": "local-only; not OSS/S3 qualification",
  "operations": ["put_object", "get_object", "delete_object"],
  "checks": {"round_trip": true, "corruption_rejected": true, "deleted": true}
}
```

It exits unsuccessfully if any check fails. `MemoryBucket` stores only in memory;
it is neither an OSS emulator nor evidence of actual service behavior. Tests
use a separate recording/fault-injecting bucket and disable Python socket
construction during the demo test. That test is not native network isolation.

## Application contract

`DocumentStore(bucket, collection=..., max_bytes=...)` takes an explicitly
supplied legacy `oss2.Bucket`. The SDK import is type-checking-only, so importing
the application does not discover credentials, construct a client or connect.
`requirements-oss.txt` records the candidate source dependency `oss2==2.18.3`;
it is **not installed, transitively locked or runtime-qualified by this work**.
An actual OSS client must be supplied only in a separately authorized environment.

- `save(id, body, filename=..., content_type=...)` uploads bytes and returns an
  immutable `Document`. Existing ID content and metadata are replaced together
  in one object PUT. There is no concurrency/version conflict protection.
- `load(id)` returns that document only after validating metadata, bounded
  content length and SHA-256. Download streams close on success and failure.
- `delete(id)` targets only its scoped key. Repeated deletion is supported by
  the local stand-in; real deletion/versioning semantics remain unverified.
- Key: `documents/{collection}/{id}`. Collection and ID are 1–64 ASCII letters,
  digits, `_` or `-`, beginning with a letter/digit. Prefixes organize data;
  they are not authenticated tenant isolation.
- Body: bytes, including empty/binary content. Default maximum is 1 MiB;
  configurable positive integer. Reads use at most 64 KiB chunks and stop at
  the limit plus one detection byte, including absent/incorrect length headers.
  This application bound is not a Spec 2 CPU/memory/deadline budget.
- Filename: printable, path-free, at most 255 UTF-8 bytes. MIME type has no
  parameters. Filename metadata is strict base64 UTF-8; digest metadata is
  lowercase SHA-256. HTTP metadata header names are read case-insensitively.
- No ETag-as-MD5 assumption. A stored hash detects accidental inconsistency,
  **not authenticity**: an actor replacing both bytes and metadata can forge it.
- Validation failures occur before storage calls. SDK exceptions propagate;
  the application performs no retry or automatic cleanup writes. Unexpected
  write status is not success; the remote write may nevertheless have happened.
  SDK/internal transport retries must be separately bounded before live use.

## Complete source call inventory

| Source location | Actual OSS call | Inputs / outputs and dependencies |
|---|---|---|
| `document_store.py`, `DocumentStore.save` | `self.bucket.put_object(key, body, headers=...)` | Deterministic collection/ID key; bytes; `Content-Type`, `x-oss-meta-filename-b64`, `x-oss-meta-sha256`; require status 200 |
| `document_store.py`, `DocumentStore.load` | `self.bucket.get_object(key)` | Require status 200; inspect `headers`/`content_length`, bounded `read(size)`, always `close()`; corrupt content/metadata rejected |
| `document_store.py`, `DocumentStore.delete` | `self.bucket.delete_object(key)` | Same key identity; require status 204; exceptions propagate |

Only these three operation types are in this candidate. No listing, signing,
ACL, multipart, encryption, service setup, retry orchestration or dependency
discovery is performed. The bucket constructor/configuration is caller-owned
and deliberately absent; its endpoint, signing mode, permissions, versions and
transport limits must become explicit inputs before real-service use.

Local code inventory: `document_store.py` (application), `local_demo.py`
(stand-in/entrypoint), `test_document_store.py` (development tests). None imports
another application or dynamically discovers SDK calls. SHA-256 file identities
and the TDD record are in the existing workflow's
[workload report](../../.workflow/ultracode/python-oss-to-s3-pack/results/reference-workload-2026-10-30.md).
They do not replace the required repository commit or independent selection.

## SDK evidence and remaining gates

The legacy API shape is checked against Alibaba's
[2.18.3 API source](https://github.com/aliyun/aliyun-oss-python-sdk/blob/2.18.3/oss2/api.py)
and [result models](https://github.com/aliyun/aliyun-oss-python-sdk/blob/2.18.3/oss2/models.py):
`GetObjectResult` exposes `headers`, nullable `content_length`, `read(amt)` and
`close()`. The downloaded models file has SHA-256
`bd199f30ef0fc8762533ce998d92ceebf72c3dfb8de6bb01fadfca4806f63330`.
The [version declaration](https://github.com/aliyun/aliyun-oss-python-sdk/blob/2.18.3/oss2/__init__.py)
matches 2.18.3. Context7 returned the incompatible V2 SDK; it was not substituted.
No downloaded SDK source was executed or copied into this app.

Local execution was tested on Python 3.14.4; older Python and installed-SDK
compatibility are unverified. The SDK's
[setup declarations](https://github.com/aliyun/aliyun-oss-python-sdk/blob/2.18.3/setup.py)
include unresolved transitive ranges; a single exact top-level requirement is
not a frozen environment. No boto3 version or equivalence claim is selected.

Still open: final non-toy/fixture-license/lineage and timed inventory approval,
owner-admitted immutable baseline, actual SDK environments, independent corpus and
oracle, upstream applicability/authenticated execution, authorized service
profile, migration implementation and reproduction. Developer tests must never
be relabelled as that independent oracle. No AI-assisted runtime is present;
this is not an approved UD-4 no-AI decision.
