# Products service

The Docker image includes the PDF runtime (Python 3, ReportLab and DejaVu fonts).
For running the Rust service directly, install these dependencies and optionally set:

- `CERTIFICATE_PYTHON`: absolute path to a Python executable with ReportLab installed (default: `python3`).
- `CERTIFICATE_FONT_DIR`: directory containing `DejaVuSans.ttf` and `DejaVuSans-Bold.ttf` (default: `/usr/share/fonts/truetype/dejavu`).
- `READ_MODELS_URL`: CQRS service base URL (default: `http://read-models-service:3006`).
- `PRODUCTION_SERVICE_URL`: production service base URL (default: `http://productions-service:3004`).

Public provenance reads the CQRS projection using short-lived service tokens scoped to the product and business. Product visibility is checked synchronously; batch ownership is checked against the production service when saving a product. Tokens never appear in API responses. Material receipt and expiry dates are copied into the production record when a material is added. Old records with missing dates remain explicitly marked as not recorded. See [messaging and public QR setup](../../docs/messaging-cqrs.md).

Regression checks from the repository root, with Docker Compose running:

```text
node scripts/verify-business-trace.cjs
node scripts/verify-role-stock.cjs
```

The first check creates and removes disposable accounts and business data, and writes a sample PDF to `tmp/pdfs/traceability-check.pdf` for visual inspection.
