use axum::{response::Response, Extension, Json};
use bigdecimal::BigDecimal;
use common::{
    errors::{AppError, AppResult},
    middleware::{require_business, AuthClaims},
    response::no_content,
};
use serde::Deserialize;
use sqlx::{PgPool, Row};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Consumption {
    operation_id: Uuid,
    raw_material_id: Uuid,
    quantity: f64,
    unit: String,
}

/// The whole production request succeeds or rolls back. Operation IDs make retries safe.
pub async fn consume(
    AuthClaims(claims): AuthClaims,
    Extension(pool): Extension<PgPool>,
    Json(mut items): Json<Vec<Consumption>>,
) -> AppResult<Response> {
    common::service_auth::require(&claims,"MATERIAL_STOCK",Uuid::nil())?;
    let business_id = require_business(&claims)?;
    items.sort_by_key(|i| i.raw_material_id);
    let mut tx = pool.begin().await?;
    for item in items {
        sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,33))").bind(item.operation_id.to_string()).execute(&mut *tx).await?;
        let released:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM consumption_tombstones WHERE operation_id=$1)").bind(item.operation_id).fetch_one(&mut *tx).await?;
        if released { return Err(AppError::Conflict("Consumption has been released".into())); }
        if !item.quantity.is_finite()
            || item.quantity < 0.001
            || item.unit.trim().is_empty()
            || (item.quantity * 1000.0 - (item.quantity * 1000.0).round()).abs() > 0.000001
        {
            return Err(AppError::BadRequest("Quantity must be at least 0.001, have at most 3 decimal places, and a unit is required".into()));
        }
        let quantity = BigDecimal::from_str(&item.quantity.to_string())
            .map_err(|_| AppError::BadRequest("Invalid quantity".into()))?;
        let inserted = sqlx::query("INSERT INTO production_consumption (operation_id, business_id, raw_material_id, quantity, unit) VALUES ($1,$2,$3,$4,$5) ON CONFLICT DO NOTHING")
            .bind(item.operation_id).bind(business_id).bind(item.raw_material_id).bind(&quantity).bind(item.unit.trim())
            .execute(&mut *tx).await?.rows_affected();
        if inserted == 0 {
            let row = sqlx::query("SELECT business_id, raw_material_id, quantity, unit, released FROM production_consumption WHERE operation_id=$1 FOR UPDATE")
                .bind(item.operation_id).fetch_one(&mut *tx).await?;
            if row.get::<Uuid, _>("business_id") != business_id
                || row.get::<Uuid, _>("raw_material_id") != item.raw_material_id
                || row.get::<BigDecimal, _>("quantity") != quantity
                || row.get::<String, _>("unit") != item.unit.trim()
                || row.get::<bool, _>("released")
            {
                return Err(AppError::Conflict(
                    "Consumption operation already used".into(),
                ));
            }
            continue;
        }
        let rows = sqlx::query("UPDATE raw_materials SET quantity=quantity-$1 WHERE id=$2 AND business_id=$3 AND NOT is_deleted AND unit=$4 AND quantity >= $1")
            .bind(&quantity).bind(item.raw_material_id).bind(business_id).bind(item.unit.trim())
            .execute(&mut *tx).await?.rows_affected();
        if rows == 0 {
            return Err(AppError::BadRequest(
                "Not enough stock, mismatched unit, or material unavailable for this business".into(),
            ));
        }
    }
    tx.commit().await?;
    Ok(no_content())
}

/// Compensates a failed production write; historical links without a ledger entry do not add stock.
pub async fn release(
    AuthClaims(claims): AuthClaims,
    Extension(pool): Extension<PgPool>,
    Json(mut ids): Json<Vec<Uuid>>,
) -> AppResult<Response> {
    common::service_auth::require(&claims,"MATERIAL_STOCK",Uuid::nil())?;
    let business_id = require_business(&claims)?;
    ids.sort();
    let mut tx = pool.begin().await?;
    for id in ids {
        sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,33))").bind(id.to_string()).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO consumption_tombstones(operation_id,business_id) VALUES($1,$2) ON CONFLICT DO NOTHING").bind(id).bind(business_id).execute(&mut *tx).await?;
        if let Some(row) = sqlx::query("UPDATE production_consumption SET released=TRUE WHERE operation_id=$1 AND business_id=$2 AND NOT released RETURNING raw_material_id, quantity")
            .bind(id).bind(business_id).fetch_optional(&mut *tx).await? {
            sqlx::query("UPDATE raw_materials SET quantity=quantity+$1 WHERE id=$2 AND business_id=$3")
                .bind(row.get::<BigDecimal,_>("quantity")).bind(row.get::<Uuid,_>("raw_material_id")).bind(business_id)
                .execute(&mut *tx).await?;
        }
    }
    tx.commit().await?;
    Ok(no_content())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::jwt::Claims;

    fn claims(business_id: Uuid, role: &str) -> AuthClaims {
        AuthClaims(Claims {
            sub: Uuid::nil(),
            email: "test@example.com".into(),
            role: role.into(),
            business_id: Some(business_id),
            exp: usize::MAX,
            iat: 0,
        })
    }
    fn item(id: Uuid, material: Uuid, quantity: f64) -> Consumption {
        Consumption {
            operation_id: id,
            raw_material_id: material,
            quantity,
            unit: "kg".into(),
        }
    }
    async fn quantity(pool: &PgPool, id: Uuid) -> BigDecimal {
        sqlx::query_scalar("SELECT quantity FROM raw_materials WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap()
    }

    #[tokio::test]
    #[ignore = "requires DATABASE_URL; uses connection-local temporary tables only"]
    async fn consumption_is_atomic_scoped_and_idempotent() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .idle_timeout(None)
            .max_lifetime(None)
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        sqlx::raw_sql("CREATE TEMP TABLE raw_materials (id UUID PRIMARY KEY, business_id UUID NOT NULL, quantity NUMERIC(12,3), unit TEXT, is_deleted BOOLEAN DEFAULT FALSE)").execute(&pool).await.unwrap();
        let migration = include_str!("../../migrations/20260914000000_production_consumption.sql")
            .replace("CREATE TABLE", "CREATE TEMP TABLE");
        sqlx::raw_sql(&migration).execute(&pool).await.unwrap();
        sqlx::raw_sql("CREATE TEMP TABLE consumption_tombstones (operation_id UUID PRIMARY KEY, business_id UUID NOT NULL, created_at TIMESTAMPTZ DEFAULT now())").execute(&pool).await.unwrap();
        let business = Uuid::new_v4();
        let material = Uuid::new_v4();
        let second = Uuid::new_v4();
        let op = Uuid::new_v4();
        for id in [material, second] {
            sqlx::query(
                "INSERT INTO raw_materials(id,business_id,quantity,unit) VALUES ($1,$2,10,'kg')",
            )
            .bind(id)
            .bind(business)
            .execute(&pool)
            .await
            .unwrap();
        }
        consume(
            claims(business, "MATERIAL_STOCK"),
            Extension(pool.clone()),
            Json(vec![item(op, material, 3.0)]),
        )
        .await
        .unwrap();
        consume(
            claims(business, "MATERIAL_STOCK"),
            Extension(pool.clone()),
            Json(vec![item(op, material, 3.0)]),
        )
        .await
        .unwrap();
        assert_eq!(quantity(&pool, material).await, BigDecimal::from(7));
        assert!(consume(
            claims(business, "MATERIAL_STOCK"),
            Extension(pool.clone()),
            Json(vec![
                item(Uuid::new_v4(), material, 2.0),
                item(Uuid::new_v4(), second, 11.0)
            ])
        )
        .await
        .is_err());
        assert_eq!(quantity(&pool, material).await, BigDecimal::from(7));
        assert_eq!(quantity(&pool, second).await, BigDecimal::from(10));
        assert!(consume(
            claims(Uuid::new_v4(), "MATERIAL_STOCK"),
            Extension(pool.clone()),
            Json(vec![item(Uuid::new_v4(), material, 1.0)])
        )
        .await
        .is_err());
        assert!(consume(
            claims(business, "CUSTOMER"),
            Extension(pool.clone()),
            Json(vec![item(Uuid::new_v4(), material, 1.0)])
        )
        .await
        .is_err());
        let mut wrong_unit = item(Uuid::new_v4(), material, 1.0);
        wrong_unit.unit = "L".into();
        assert!(consume(
            claims(business, "MATERIAL_STOCK"),
            Extension(pool.clone()),
            Json(vec![wrong_unit])
        )
        .await
        .is_err());
        for invalid in [-1.0, 0.0, 0.0009, 1.2345] {
            assert!(consume(
                claims(business, "MATERIAL_STOCK"),
                Extension(pool.clone()),
                Json(vec![item(Uuid::new_v4(), material, invalid)])
            )
            .await
            .is_err());
        }
        release(
            claims(business, "MATERIAL_STOCK"),
            Extension(pool.clone()),
            Json(vec![op]),
        )
        .await
        .unwrap();
        release(
            claims(business, "MATERIAL_STOCK"),
            Extension(pool.clone()),
            Json(vec![op]),
        )
        .await
        .unwrap();
        assert_eq!(quantity(&pool, material).await, BigDecimal::from(10));
        pool.close().await;
    }
}
