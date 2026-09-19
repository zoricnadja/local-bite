use anyhow::Context;
use image::Luma;
use qrcode::QrCode;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Generate a QR code PNG for the given product token.
/// Returns the relative path where the file was saved (e.g. "qr/abc123.png").
pub fn generate_qr(qr_token: Uuid, uploads_dir: &str) -> anyhow::Result<String> {
    // The URL a customer will land on after scanning
    // QR codes lead to the public Angular traceability screen rather than a raw
    // JSON API response. PUBLIC_TRACE_URL is set to the externally reachable
    // address in deployment (for example https://localbite.rs/trace).
    let trace_url = std::env::var("PUBLIC_TRACE_URL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "http://localhost/trace".into());
    let target_url = format!("{}/{}", trace_url.trim_end_matches('/'), qr_token);

    // URL-specific filenames invalidate images created with an old host or port.
    let mut hash = std::collections::hash_map::DefaultHasher::new();
    target_url.hash(&mut hash);
    let filename = format!("{}-{:x}.png", qr_token, hash.finish());
    let qr_dir = PathBuf::from(uploads_dir).join("qr");
    std::fs::create_dir_all(&qr_dir)?;
    let full_path = qr_dir.join(&filename);
    if full_path.is_file() {
        return Ok(format!("qr/{}", filename));
    }

    // Build QR code
    let code = QrCode::new(target_url.as_bytes()).context("Failed to create QR code")?;

    // Render as image (each module = 10px, quiet zone = 4 modules)
    let image = code
        .render::<Luma<u8>>()
        .min_dimensions(300, 300)
        .quiet_zone(true)
        .build();

    image.save(&full_path).context("Failed to save QR PNG")?;

    Ok(format!("qr/{}", filename))
}

/// Returns the full filesystem path for a relative media path.
pub fn media_path(relative: &str, uploads_dir: &str) -> PathBuf {
    Path::new(uploads_dir).join(relative)
}
