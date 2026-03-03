use anyhow::{anyhow, Context};
use uuid::Uuid;
use crate::dtos::product::product_api_data::ProductApiData;
use crate::dtos::product::product_api_response::ProductApiResponse;

pub async fn fetch_product(product_id: Uuid, token: &str) -> anyhow::Result<ProductApiData> {
    let base = std::env::var("PRODUCTS_SERVICE_URL")
        .unwrap_or_else(|_| "http://products-service:3003".into());
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

    Ok(body.data)
}
