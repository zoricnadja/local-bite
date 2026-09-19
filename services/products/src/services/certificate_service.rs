use std::sync::Arc;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use crate::services::provenance_service::ProvenanceService;
use common::errors::{AppError, AppResult};

#[derive(Clone)]
pub struct CertificateService {
    provenance_service: Arc<ProvenanceService>,
}

impl CertificateService {
    pub fn new(provenance_service: Arc<ProvenanceService>) -> Self {
        Self { provenance_service }
    }

    pub async fn generate(&self, qr_token: Uuid) -> AppResult<Vec<u8>> {
        let provenance = self.provenance_service.get_provenance_by_qr(qr_token).await?;
        let provenance=crate::dtos::public_provenance::PublicProvenance::from(provenance);
        let payload = serde_json::to_vec(&provenance).map_err(|e| AppError::Internal(e.into()))?;
        let render = async {
            // Structured stdin only; no user-controlled shell commands or file paths.
            let mut child = tokio::process::Command::new(
                std::env::var("CERTIFICATE_PYTHON").unwrap_or_else(|_| "python3".into()))
                .arg("-c").arg(include_str!("../../assets/certificate.py"))
                .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
                .kill_on_drop(true).spawn()?;
            let mut stdin = child.stdin.take().ok_or_else(|| anyhow::anyhow!("PDF renderer stdin unavailable"))?;
            stdin.write_all(&payload).await?;
            drop(stdin);
            let output = child.wait_with_output().await?;
            if !output.status.success() || !output.stdout.starts_with(b"%PDF-") {
                tracing::error!("PDF renderer failed");
                return Err(anyhow::anyhow!("Could not generate traceability record"));
            }
            Ok(output.stdout)
        };
        tokio::time::timeout(std::time::Duration::from_secs(20), render).await
            .map_err(|_| AppError::Internal(anyhow::anyhow!("PDF generation timed out")))?
            .map_err(AppError::Internal)
    }
}
