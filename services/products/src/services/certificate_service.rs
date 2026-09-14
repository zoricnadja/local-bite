use std::sync::Arc;

use uuid::Uuid;

use crate::dtos::provenance_response::ProvenanceResponse;
use crate::services::provenance_service::ProvenanceService;
use common::errors::AppResult;

/// Builder pattern: keeps presentation concerns out of the provenance query and
/// makes it straightforward to add HTML or e-signature certificate renderers.
pub struct CertificateDocumentBuilder {
    lines: Vec<String>,
}

impl CertificateDocumentBuilder {
    pub fn new() -> Self {
        Self {
            lines: vec!["LOCAL BITE - CERTIFICATE OF ORIGIN".into()],
        }
    }
    pub fn product(mut self, provenance: &ProvenanceResponse) -> Self {
        let p = &provenance.product;
        self.lines
            .push(format!("Product: {} ({})", p.name, p.product_type));
        self.lines.push(format!(
            "Producer: {}",
            provenance.farm_name.as_deref().unwrap_or("Not available")
        ));
        self.lines
            .push(format!("Quantity: {} {}", p.quantity, p.unit));
        self.lines.push(format!("Traceability ID: {}", p.qr_token));
        self
    }
    pub fn production(mut self, provenance: &ProvenanceResponse) -> Self {
        if let Some(batch) = &provenance.batch {
            self.lines.push(format!(
                "Production batch: {} / {}",
                batch.name, batch.process_type
            ));
            self.lines.push(format!("Status: {}", batch.status));
            if !batch.raw_materials.is_empty() {
                self.lines.push(format!(
                    "Raw materials: {}",
                    batch
                        .raw_materials
                        .iter()
                        .map(|m| m.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        } else {
            self.lines.push("Production batch: not linked".into());
        }
        self
    }
    pub fn build(self) -> Vec<u8> {
        minimal_pdf(&self.lines)
    }
}

#[derive(Clone)]
pub struct CertificateService {
    provenance_service: Arc<ProvenanceService>,
}

impl CertificateService {
    pub fn new(provenance_service: Arc<ProvenanceService>) -> Self {
        Self { provenance_service }
    }
    pub async fn generate(&self, qr_token: Uuid) -> AppResult<Vec<u8>> {
        let provenance = self
            .provenance_service
            .get_provenance_by_qr(qr_token)
            .await?;
        Ok(CertificateDocumentBuilder::new()
            .product(&provenance)
            .production(&provenance)
            .build())
    }
}

/// A deliberately small dependency-free PDF renderer for a single certificate
/// page. Dynamic text is escaped, so user supplied product data cannot break the
/// document syntax.
fn minimal_pdf(lines: &[String]) -> Vec<u8> {
    let content = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let escaped = line
                .replace('\\', "\\\\")
                .replace('(', "\\(")
                .replace(')', "\\)");
            format!(
                "BT /F1 {} Tf 54 {} Td ({}) Tj ET",
                if i == 0 { 18 } else { 11 },
                780 - i as i32 * 28,
                escaped
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let objects = vec![
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>".to_string(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
        format!("<< /Length {} >>\nstream\n{}\nendstream", content.as_bytes().len(), content),
    ];
    let mut out = b"%PDF-1.4\n".to_vec();
    let mut offsets = vec![0usize];
    for (i, object) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{}\nendobj\n", i + 1, object).as_bytes());
    }
    let xref = out.len();
    out.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes(),
    );
    for offset in offsets.iter().skip(1) {
        out.extend_from_slice(format!("{:010} 00000 n \n", offset).as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            objects.len() + 1,
            xref
        )
        .as_bytes(),
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn renders_a_valid_pdf_header() {
        assert!(minimal_pdf(&["test".into()]).starts_with(b"%PDF-1.4"));
    }
}
