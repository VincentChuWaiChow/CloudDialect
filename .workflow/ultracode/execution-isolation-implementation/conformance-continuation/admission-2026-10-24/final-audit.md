# Admission audit — verified bounded slice

| Criterion | Result | Evidence |
| --- | --- | --- |
| Exact41pairs /25bounded batches | PASS | contract.json; per-group batch results;82hashes |
| Full Rust checks after every batch | PASS |25records,286tests each, observed cumulative counts |
| Final Cargo + canonical42 | PASS | integrated-checks.json |
| Prefix counts / zero selection / wrong expectations | PASS |12canonical controls |
| Check/Clippy/fmt / repository Python | PASS | integrated logs,15Python |
| Positive13/task18 content | PASS | Independent31 + actual Rust/prefix checks |
| Negative23/task19 content | PASS | Independent32 +22Fail/1Warn exact oracles |
| Roundtrip/integration full task21 | PARTIAL |3typed +3nonnative; native/P0cases remain |
| Production/lock/schema/research preservation | PASS |90protected baseline files; scope audit and independent after-hash verification pass |
| Independent integrated reviewer33 | PASS | Full286Rust+42oracles;9controls;180hashes; prefix/zero/wrong-oracle controls |
| Fresh coverage/Rust1.88 | UNVERIFIED | tools not installed; no installation authorized |
| Additional legacy spec-checker | FAIL pre-existing |8pass/1error; unchanged blanket checkbox predicate |
| Full104-case / traceability/consumer/native | MISS |62cases and required qualification outstanding |
| All21Spec2tasks/UD-BG gates | MISS |0numbered runtime tasks complete; no execution crate |

No default full-goal completion, native isolation, authentication, acceptance or
publication claim follows from this slice. No source/agent repair was applied.
