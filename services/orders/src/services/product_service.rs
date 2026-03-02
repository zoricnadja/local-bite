use anyhow::{anyhow, Context};
use uuid::Uuid;
use crate::dtos::product::product_api_response::ProductApiResponse;
use crate::dtos::product::product_snapshot::ProductSnapshot;

pub async fn fetch_product(product_id: Uuid, token: &str) -> anyhow::Result<ProductSnapshot> {
    let base = std::env::var("PRODUCT_SERVICE_URL")
        .unwrap_or_else(|_| "http://product:3004".into());
    let url = format!("{}/products/{}", base, product_id);

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .context("Failed to reach product service")?;

    if resp.status().as_u16() == 404 {
        return Err(anyhow!("Product {} not found or not owned by your farm", product_id));
    }
    if !resp.status().is_success() {
        return Err(anyhow!(
            "Product service returned {} for product {}",
            resp.status(), product_id
        ));
    }

    let body: ProductApiResponse = resp
        .json()
        .await
        .context("Failed to parse product response")?;

    Ok(ProductSnapshot {
        id:           body.data.id,
        name:         body.data.name,
        product_type: body.data.product_type,
        price:        body.data.price,
        unit:         body.data.unit,
        is_active:    body.data.is_active,
    })
}
