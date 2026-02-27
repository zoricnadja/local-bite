/// Thin async HTTP clients for calling other microservices.
/// Each function takes the base URL from env and a JWT token (forwarded from the caller).
use anyhow::{anyhow, Context};
use serde::Deserialize;
use uuid::Uuid;
use crate::models::batch_ref::BatchRef;
use crate::models::process_step_ref::ProcessStepRef;
use crate::models::raw_material_ref::RawMaterialRef;
// ── Auth Service ──────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct FarmResponse {
    data: FarmData,
}

#[derive(Deserialize)]
struct FarmData {
    name: String,
}

pub async fn fetch_farm_name(farm_id: Uuid, token: &str) -> Option<String> {
    let base = std::env::var("AUTH_SERVICE_URL").unwrap_or_else(|_| "http://auth-service:3001".into());
    let url = format!("{}/farms/{}", base, farm_id);

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let body: FarmResponse = resp.json().await.ok()?;
    Some(body.data.name)
}

// ── Production Service ────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct BatchApiResponse {
    data: BatchApiData,
}

#[derive(Deserialize)]
struct BatchApiData {
    id:           Uuid,
    name:         String,
    process_type: String,
    start_date:   Option<String>,
    end_date:     Option<String>,
    status:       String,
    steps:        Vec<StepApiData>,
    raw_materials: Vec<RawMaterialApiData>,
}

#[derive(Deserialize)]
struct StepApiData {
    id:             Uuid,
    step_order:     i32,
    name:           String,
    description:    Option<String>,
    duration_hours: Option<f64>,
    temperature:    Option<f64>,
}

#[derive(Deserialize)]
struct RawMaterialApiData {
    id:            Uuid,
    name:          String,
    material_type: String,
    quantity_used: f64,
    unit:          String,
    origin:        Option<String>,
    supplier:      Option<String>,
}

pub async fn fetch_batch(batch_id: Uuid, token: &str) -> anyhow::Result<BatchRef> {
    let base = std::env::var("PRODUCTION_SERVICE_URL")
        .unwrap_or_else(|_| "http://production:3003".into());
    let url = format!("{}/batches/{}", base, batch_id);

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .context("Failed to reach production service")?;

    if !resp.status().is_success() {
        return Err(anyhow!("Production service returned {}", resp.status()));
    }

    let body: BatchApiResponse = resp.json().await.context("Failed to parse batch response")?;
    let d = body.data;

    Ok(BatchRef {
        id:           d.id,
        name:         d.name,
        process_type: d.process_type,
        start_date:   d.start_date,
        end_date:     d.end_date,
        status:       d.status,
        steps: d.steps.into_iter().map(|s| ProcessStepRef {
            id:             s.id,
            step_order:     s.step_order,
            name:           s.name,
            description:    s.description,
            duration_hours: s.duration_hours,
            temperature:    s.temperature,
        }).collect(),
        raw_materials: d.raw_materials.into_iter().map(|r| RawMaterialRef {
            id:            r.id,
            name:          r.name,
            material_type: r.material_type,
            quantity_used: r.quantity_used,
            unit:          r.unit,
            origin:        r.origin,
            supplier:      r.supplier,
        }).collect(),
    })
}
