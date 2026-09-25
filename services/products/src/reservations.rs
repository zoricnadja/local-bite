use axum::{extract::Path, Extension, Json, response::Response};
use bigdecimal::BigDecimal;
use common::{errors::{AppError, AppResult}, middleware::AuthClaims, response::no_content, service_auth};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct Item { pub product_id: Uuid, pub quantity: String, pub unit_price: String }

pub async fn reserve(AuthClaims(claims): AuthClaims, Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>, Json(mut items): Json<Vec<Item>>) -> AppResult<Response> {
    service_auth::require(&claims, "ORDER_STOCK", id)?;
    items.sort_by_key(|i| i.product_id);
    if items.is_empty() || items.windows(2).any(|w|w[0].product_id==w[1].product_id) {
        return Err(AppError::BadRequest("Unique reservation items are required".into()));
    }
    let payload = serde_json::to_value(&items).map_err(|e|AppError::Internal(e.into()))?;
    let mut tx=pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,11))").bind(id.to_string()).execute(&mut *tx).await?;
    if let Some(existing)=sqlx::query_scalar::<_,serde_json::Value>("SELECT payload FROM stock_reservations WHERE id=$1").bind(id).fetch_optional(&mut *tx).await? {
        if existing!=payload { return Err(AppError::Conflict("Reservation key has different items".into())); }
        tx.commit().await?; return Ok(no_content());
    }
    sqlx::query("INSERT INTO stock_reservations(id,payload) VALUES($1,$2)").bind(id).bind(payload).execute(&mut *tx).await?;
    for item in items {
        let quantity=BigDecimal::from_str(&item.quantity).map_err(|_|AppError::BadRequest("Invalid quantity".into()))?;
        let price=BigDecimal::from_str(&item.unit_price).map_err(|_|AppError::BadRequest("Invalid price".into()))?;
        if quantity<=BigDecimal::from(0) || quantity!=quantity.with_scale(3) { return Err(AppError::BadRequest("Quantity must be positive with at most three decimals".into())); }
        let row=sqlx::query("UPDATE products p SET quantity=p.quantity-$2 WHERE p.id=$1 AND NOT p.is_deleted AND p.is_active AND p.quantity >= $2 AND p.price=$3 AND p.price>0 AND (p.expiry_date IS NULL OR p.expiry_date>=CURRENT_DATE) AND EXISTS(SELECT 1 FROM production_states b WHERE b.batch_id=p.batch_id AND b.business_id=p.business_id AND b.status='COMPLETED' AND NOT b.deleted) RETURNING business_id")
            .bind(item.product_id).bind(&quantity).bind(price).fetch_optional(&mut *tx).await?
            .ok_or_else(||AppError::Conflict("Product unavailable, price changed or insufficient stock".into()))?;
        sqlx::query("INSERT INTO stock_reservation_items(reservation_id,product_id,business_id,quantity) VALUES($1,$2,$3,$4)")
            .bind(id).bind(item.product_id).bind(row.get::<Uuid,_>("business_id")).bind(quantity).execute(&mut *tx).await?;
    }
    tx.commit().await?; Ok(no_content())
}

pub async fn release(AuthClaims(claims):AuthClaims,Extension(pool):Extension<PgPool>,Path((id,business)):Path<(Uuid,Uuid)>)->AppResult<Response>{
    service_auth::require(&claims,"ORDER_STOCK",id)?;
    if claims.business_id!=Some(business){return Err(AppError::Forbidden("Reservation business mismatch".into()));}
    let mut tx=pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,11))").bind(id.to_string()).execute(&mut *tx).await?;
    let rows=sqlx::query("SELECT product_id,quantity FROM stock_reservation_items WHERE reservation_id=$1 AND business_id=$2 AND NOT released ORDER BY product_id FOR UPDATE")
        .bind(id).bind(business).fetch_all(&mut *tx).await?;
    for row in rows {
        let product:Uuid=row.get("product_id");
        sqlx::query("UPDATE products SET quantity=quantity+$2 WHERE id=$1").bind(product).bind(row.get::<BigDecimal,_>("quantity")).execute(&mut *tx).await?;
        sqlx::query("UPDATE stock_reservation_items SET released=true WHERE reservation_id=$1 AND product_id=$2").bind(id).bind(product).execute(&mut *tx).await?;
    }
    tx.commit().await?;Ok(no_content())
}
