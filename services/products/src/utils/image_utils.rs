use anyhow::Context;
use image::imageops::FilterType;
use std::path::PathBuf;
use uuid::Uuid;

const MAX_DIMENSION: u32 = 800;
const MAX_FILE_BYTES: usize = 10 * 1024 * 1024; // 10 MB

/// Allowed MIME types for product images.
pub fn is_allowed_mime(mime: &str) -> bool {
    matches!(mime, "image/jpeg" | "image/png" | "image/webp")
}

/// Save an uploaded image: validate size, decode, resize to ≤800x800, save as JPEG.
/// Returns the relative path (e.g. "images/uuid.jpg").
pub fn save_image(bytes: &[u8], uploads_dir: &str) -> anyhow::Result<String> {
    if bytes.len() > MAX_FILE_BYTES {
        anyhow::bail!("Image too large (max 10 MB)");
    }

    // Decode
    let img = image::load_from_memory(bytes).context("Failed to decode image")?;

    // Resize if needed (preserves aspect ratio)
    let img = if img.width() > MAX_DIMENSION || img.height() > MAX_DIMENSION {
        img.resize(MAX_DIMENSION, MAX_DIMENSION, FilterType::Lanczos3)
    } else {
        img
    };

    // Ensure images/ sub-directory exists
    let img_dir = PathBuf::from(uploads_dir).join("images");
    std::fs::create_dir_all(&img_dir)?;

    let filename = format!("{}.jpg", Uuid::new_v4());
    let full_path = img_dir.join(&filename);

    img.save_with_format(&full_path, image::ImageFormat::Jpeg)
        .context("Failed to save image")?;

    Ok(format!("images/{}", filename))
}

/// Delete an image file from disk (best-effort, ignores errors).
pub fn delete_image(relative_path: &str, uploads_dir: &str) {
    let full = PathBuf::from(uploads_dir).join(relative_path);
    let _ = std::fs::remove_file(full);
}
