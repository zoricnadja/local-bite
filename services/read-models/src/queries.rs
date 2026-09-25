use axum::{Extension, extract::Path, response::Response};
use common::{errors::{AppError, AppResult}, middleware::{AuthClaims, require_business, require_role}, response::ok};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

async fn entity(pool: &PgPool, kind: &str, id: Uuid) -> AppResult<Option<Value>> {
    Ok(sqlx::query_scalar("SELECT data FROM projection_entities WHERE entity_type=$1 AND entity_id=$2 AND NOT deleted")
        .bind(kind).bind(id).fetch_optional(pool).await?)
}

async fn business_entities(pool: &PgPool, kind: &str, business: Option<Uuid>) -> AppResult<Vec<Value>> {
    Ok(sqlx::query_scalar("SELECT data FROM projection_entities WHERE entity_type=$1 AND NOT deleted AND ($2::text IS NULL OR data->>'business_id'=$2) ORDER BY data->>'created_at' DESC,entity_id")
        .bind(kind).bind(business.map(|id| id.to_string())).fetch_all(pool).await?)
}

pub async fn dashboard(AuthClaims(claims): AuthClaims, Extension(pool): Extension<PgPool>) -> AppResult<Response> {
    require_role(&claims, &["CUSTOMER", "WORKER", "BUSINESS_OWNER", "SYSTEM_ADMIN"])?;
    let as_of: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar("SELECT max(applied_at) FROM projection_receipts").fetch_one(&pool).await?;
    if claims.role == "CUSTOMER" {
        let orders: Vec<Value> = sqlx::query_scalar("SELECT data FROM projection_entities WHERE entity_type='orders' AND NOT deleted AND data->>'customer_id'=$1 ORDER BY data->>'created_at' DESC,entity_id LIMIT 5")
            .bind(claims.sub.to_string()).fetch_all(&pool).await?;
        return Ok(ok(json!({"recentOrders":orders,"asOf":as_of})));
    }
    let business = if claims.role == "SYSTEM_ADMIN" { None } else { Some(require_business(&claims)?) };
    let (materials, batches, products, orders) = tokio::try_join!(
        business_entities(&pool,"raw_materials",business), business_entities(&pool,"production_batches",business),
        business_entities(&pool,"products",business), business_entities(&pool,"orders",business))?;
    let low_stock: Vec<&Value> = materials.iter().filter(|m| !m["low_stock_threshold"].is_null() && number(&m["quantity"]) <= number(&m["low_stock_threshold"])).collect();
    let active: Vec<&Value> = batches.iter().filter(|b| b["status"]=="IN_PROGRESS").collect();
    let planned: Vec<&Value> = batches.iter().filter(|b| b["status"]=="PLANNED").collect();
    let revenue: f64 = orders.iter().filter(|o| o["status"]=="DELIVERED").map(|o| number(&o["total_price"])).sum();
    Ok(ok(json!({"lowStockItems":low_stock,"activeBatches":active.iter().take(5).collect::<Vec<_>>(),
        "plannedBatches":planned.iter().take(5).collect::<Vec<_>>(),"plannedCount":planned.len(),
        "stats":{"totalOrders":if claims.role=="WORKER" {0} else {orders.len()},
        "revenue":if claims.role=="WORKER" {0.0} else {revenue},"activeBatches":active.len(),"totalProducts":products.len()},"asOf":as_of})))
}

pub async fn provenance(AuthClaims(claims): AuthClaims, Path(id): Path<Uuid>, Extension(pool): Extension<PgPool>) -> AppResult<Response> {
    require_role(&claims, &["TRACEABILITY"])?;
    if claims.sub != id { return Err(AppError::Forbidden("Traceability scope mismatch".into())); }
    let product=entity(&pool,"products",id).await?.ok_or_else(|| AppError::Conflict("Traceability is synchronizing; please retry shortly".into()))?;
    let business_id=product["business_id"].as_str().and_then(|id| id.parse::<Uuid>().ok())
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Invalid projected business id")))?;
    if claims.business_id != Some(business_id) { return Err(AppError::Forbidden("Traceability scope mismatch".into())); }
    let business=entity(&pool,"businesses",business_id).await?;
    let mut batch=Value::Null;
    if let Some(batch_id)=product["batch_id"].as_str().and_then(|id| id.parse::<Uuid>().ok()) {
        if let Some(mut b)=entity(&pool,"production_batches",batch_id).await? {
            if b["business_id"] != product["business_id"] { return Err(AppError::Forbidden("Production belongs to another business".into())); }
            let materials: Vec<Value> = sqlx::query_scalar("SELECT data FROM projection_entities WHERE entity_type='batch_raw_materials' AND NOT deleted AND data->>'batch_id'=$1 AND data->>'business_id'=$2 ORDER BY entity_id")
                .bind(batch_id.to_string()).bind(business_id.to_string()).fetch_all(&pool).await?;
            let steps: Vec<Value> = sqlx::query_scalar("SELECT data FROM projection_entities WHERE entity_type='process_steps' AND NOT deleted AND data->>'batch_id'=$1 AND data->>'business_id'=$2 ORDER BY (data->>'step_order')::int,entity_id")
                .bind(batch_id.to_string()).bind(business_id.to_string()).fetch_all(&pool).await?;
            b["raw_materials"]=Value::Array(materials.into_iter().map(|m| json!({
                "id":m["raw_material_id"],"name":m["raw_material_name"],"material_type":m["material_type"],
                "quantity_used":m["quantity_used"],"unit":m["unit"],"origin":m["origin"],"supplier":m["supplier"],
                "harvest_date":m["harvest_date"],"received_date":m["received_date"],"expiry_date":m["expiry_date"]})).collect());
            b["steps"]=Value::Array(steps);
            batch=b;
        }
    }
    Ok(ok(json!({"product":product,"business_name":business.as_ref().map(|f| &f["name"]),"batch":batch})))
}

fn number(v: &Value) -> f64 { v.as_f64().or_else(|| v.as_str().and_then(|s| s.parse().ok())).unwrap_or(0.0) }

pub async fn producers(AuthClaims(claims):AuthClaims, Extension(pool):Extension<PgPool>)->AppResult<Response> {
    require_role(&claims,&["CUSTOMER","BUSINESS_OWNER","WORKER","SYSTEM_ADMIN"])?;
    let rows:Vec<Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',entity_id,'name',data->>'name') FROM projection_entities WHERE entity_type='businesses' AND NOT deleted ORDER BY data->>'name',entity_id").fetch_all(&pool).await?;
    Ok(ok(rows))
}
