use anyhow::{anyhow, Context};
use serde::Serialize;
use uuid::Uuid;

use crate::dtos::product::product_api_data::ProductApiData;
use crate::dtos::product::product_api_response::ProductApiResponse;

fn base_url() -> String {
    std::env::var("PRODUCTS_SERVICE_URL")
        .unwrap_or_else(|_| "http://products-service:3003".into())
}

// ── Fetch single product ──────────────────────────────────────────────────────

pub async fn fetch_product(product_id: Uuid, token: &str) -> anyhow::Result<ProductApiData> {
    let url = format!("{}/products/{}", base_url(), product_id);

    let resp = reqwest::Client::new()
        .get(&url)
        .bearer_auth(token)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .context("Failed to reach product service")?;

    if resp.status().as_u16() == 404 {
        return Err(anyhow!("Product {} not found", product_id));
    }
    if !resp.status().is_success() {
        return Err(anyhow!(
            "Product service returned {} for product {}",
            resp.status(), product_id
        ));
    }

    let body: ProductApiResponse = resp.json().await.context("Failed to parse product response")?;
    Ok(body.data)
}

// ── Decrement quantity ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct DecrementPayload {
    quantity: f64,
}

pub async fn decrement_product_quantity(
    product_id: Uuid,
    quantity: f64,
    token: &str,
) -> anyhow::Result<()> {
    let url = format!("{}/products/{}/decrement", base_url(), product_id);

    let resp = reqwest::Client::new()
        .patch(&url)
        .bearer_auth(token)
        .json(&DecrementPayload { quantity })
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .context("Failed to reach product service")?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Failed to decrement quantity for product {}: {}",
            product_id, resp.status()
        ));
    }

    Ok(())
}