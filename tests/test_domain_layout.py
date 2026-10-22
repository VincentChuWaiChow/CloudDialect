"""Physical document-domain boundaries; public Rust aliases have separate tests."""

from pathlib import Path
import unittest


SRC = Path(__file__).resolve().parents[1] / "crates/clouddialect-contracts/src"


class DomainLayoutTests(unittest.TestCase):
    def test_document_families_have_cohesive_owned_files(self):
        for name in (
            "mod.rs", "behavioral_contract.rs", "evidence.rs", "verification.rs",
            "references.rs", "raw_objects.rs",
        ):
            with self.subTest(name=name):
                self.assertTrue((SRC / "documents" / name).is_file())
        for domain in ("manifest", "schema", "errors", "validator"):
            self.assertTrue((SRC / domain / "mod.rs").is_file())

    def test_validator_has_no_shadow_document_model_implementations(self):
        for old in (
            "source_documents.rs", "verification_types.rs", "references.rs", "raw_objects.rs",
        ):
            self.assertFalse((SRC / "validator" / old).exists())
        facade = (SRC / "validator/mod.rs").read_text()
        self.assertIn("pub use crate::documents::", facade)


if __name__ == "__main__":
    unittest.main()
