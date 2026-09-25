"""Tests the real builder contract and renderer, without application data."""
import importlib.util
import unittest
from pathlib import Path

spec = importlib.util.spec_from_file_location("certificate", Path(__file__).resolve().parents[1] / "services/products/assets/certificate.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class RecordingBuilder(module.CertificateBuilder):
    def __init__(self): self.calls = []
    def product(self, data): self.calls.append("product")
    def production(self, data): self.calls.append("production")
    def materials(self, data): self.calls.append("materials")
    def disclaimer(self): self.calls.append("disclaimer")
    def result(self): return self.calls


class CertificateTests(unittest.TestCase):
    def test_director_orders_sections(self):
        self.assertEqual(module.CertificateDirector().build({"product": {}, "batch": {}}, RecordingBuilder()), ["product", "production", "materials", "disclaimer"])

    def test_public_unicode_and_long_content(self):
        data = {"product": {"name": "Sir Đorđević <&>", "product_type": "dairy", "qr_token": "test", "description": "Čačak Šumadija " * 100}, "business_name": "Radionica Đorđević", "batch": {"name": "Serija", "process_type": "fermentation", "status": "COMPLETED", "steps": [], "raw_materials": [{"name": "Mleko <&>", "origin": "Šumadija"}] * 50}}
        pdf = module.render(data)
        self.assertTrue(pdf.startswith(b"%PDF-"))
        self.assertGreater(len(pdf), 1000)


if __name__ == "__main__": unittest.main()
