"""Documentation consistency checks, not contract-enforceability or legal review."""

from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[1]
DOCUMENTS = ("COMMERCIAL-LICENSING.md", "COMMERCIAL-AGREEMENT-TEMPLATE.md")


class CommercialRoyaltyTests(unittest.TestCase):
    def test_same_owner_selected_resale_rate_and_payment_base(self):
        for name in DOCUMENTS:
            text = " ".join((ROOT / name).read_text().split())
            with self.subTest(document=name):
                for required in (
                    "10%", "nonexclusive resale", "Covered Receipts",
                    "payments actually collected", "actual refunds",
                    "separately collected VAT/sales taxes", "not profit",
                    "unrelated company revenue", "salaries", "marketing", "overhead",
                ):
                    self.assertIn(required.lower(), text.lower())
                self.assertNotIn("5%", text)

    def test_same_monthly_settlement_and_unfinished_contract_boundaries(self):
        for name in DOCUMENTS:
            text = " ".join((ROOT / name).read_text().split())
            with self.subTest(document=name):
                for required in (
                    "monthly", "30 calendar days after month-end",
                    "bundles", "affiliates", "audit", "currency",
                    "signed agreement", "existing agreements",
                    "legal review", "hosting", "support",
                ):
                    self.assertIn(required.lower(), text.lower())

    def test_proposal_does_not_grant_rights_or_price_all_commercial_use(self):
        guide = " ".join((ROOT / DOCUMENTS[0]).read_text().split())
        template = " ".join((ROOT / DOCUMENTS[1]).read_text().split())
        self.assertIn("No commercial rights are granted until", guide)
        self.assertIn("not a self-executing license", template)
        self.assertIn("Other commercial uses", guide)
        self.assertIn("must not be treated as a blanket 10% rate", guide)
        self.assertIn("No fee formula applies until", template)


if __name__ == "__main__":
    unittest.main()
