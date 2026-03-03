use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use crate::dtos::analytics::analytics_response::StatusCount;
use crate::dtos::order::list_orders_query::ListOrdersQuery;
use crate::models::order::Order;

#[derive(Clone)]
pub struct OrderRepository {
    pub(crate) pool: PgPool,
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ── Queries ───────────────────────────────────────────────────────────────

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Order> {
        sqlx::query_as!(
            Order,
            r#"
            SELECT id, farm_id, customer_id, customer_name, customer_email,
                   status, total_price, notes, is_deleted, created_at, updated_at
            FROM   orders
            WHERE  id = $1 AND is_deleted = FALSE
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Order {} not found", id)))
    }

    pub async fn find_all(
        &self,
        farm_id: Uuid,
        q: &ListOrdersQuery,
    ) -> AppResult<Vec<Order>> {
        let offset   = q.offset();
        let limit    = q.limit();
        let status_f = q.status.as_deref().unwrap_or("");
        let search_f = q.search.as_deref().unwrap_or("");

        let items = sqlx::query_as!(
            Order,
            r#"
            SELECT id, farm_id, customer_id, customer_name, customer_email,
                   status, total_price, notes, is_deleted, created_at, updated_at
            FROM   orders
            WHERE  farm_id    = $1
              AND  is_deleted = FALSE
              AND  ($2 = '' OR status ILIKE $2)
              AND  ($3 = '' OR customer_name  ILIKE '%' || $3 || '%'
                            OR customer_email ILIKE '%' || $3 || '%')
            ORDER  BY created_at DESC
            LIMIT  $4 OFFSET $5
            "#,
            farm_id, status_f, search_f, limit, offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }
    pub async fn find_all_by_user_id(
        &self,
        _id: Uuid,
        q: &ListOrdersQuery,
    ) -> AppResult<Vec<Order>> {
        let offset   = q.offset();
        let limit    = q.limit();
        let status_f = q.status.as_deref().unwrap_or("");
        let search_f = q.search.as_deref().unwrap_or("");

        let items = sqlx::query_as!(
            Order,
            r#"
            SELECT id, farm_id, customer_id, customer_name, customer_email,
                   status, total_price, notes, is_deleted, created_at, updated_at
            FROM   orders
            WHERE  customer_id    = $1
              AND  is_deleted = FALSE
              AND  ($2 = '' OR status ILIKE $2)
              AND  ($3 = '' OR customer_name  ILIKE '%' || $3 || '%'
                            OR customer_email ILIKE '%' || $3 || '%')
            ORDER  BY created_at DESC
            LIMIT  $4 OFFSET $5
            "#,
            _id, status_f, search_f, limit, offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    pub async fn count(
        &self,
        farm_id: Uuid,
        q: &ListOrdersQuery,
    ) -> AppResult<i64> {
        let status_f = q.status.as_deref().unwrap_or("");
        let search_f = q.search.as_deref().unwrap_or("");

        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM orders
            WHERE  farm_id    = $1
              AND  is_deleted = FALSE
              AND  ($2 = '' OR status ILIKE $2)
              AND  ($3 = '' OR customer_name  ILIKE '%' || $3 || '%'
                            OR customer_email ILIKE '%' || $3 || '%')
            "#,
            farm_id, status_f, search_f
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        Ok(count)
    }

    // ── Commands ──────────────────────────────────────────────────────────────

    pub async fn insert(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        farm_id: Uuid,
        customer_id: Uuid,
        customer_name: Option<&str>,
        customer_email: String,
        notes: Option<&str>,
        total_price: &BigDecimal,
    ) -> AppResult<Order> {
        let order = sqlx::query_as!(
            Order,
            r#"
            INSERT INTO orders
                (id, farm_id, customer_id, customer_name, customer_email, notes, total_price)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, farm_id, customer_id, customer_name, customer_email,
                      status, total_price, notes, is_deleted, created_at, updated_at
            "#,
            Uuid::new_v4(), farm_id,
            customer_id, customer_name, customer_email,
            notes, total_price
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(order)
    }

    pub async fn update_status(
        &self,
        id: Uuid,
        farm_id: Uuid,
        new_status: &str,
    ) -> AppResult<Order> {
        let updated = sqlx::query_as!(
            Order,
            r#"
            UPDATE orders SET status = $1
            WHERE  id = $2 AND farm_id = $3 AND is_deleted = FALSE
            RETURNING id, farm_id, customer_id, customer_name, customer_email,
                      status, total_price, notes, is_deleted, created_at, updated_at
            "#,
            new_status, id, farm_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Order {} not found", id)))?;

        Ok(updated)
    }

    pub async fn soft_delete(&self, id: Uuid, farm_id: Uuid) -> AppResult<()> {
        let rows = sqlx::query!(
            "UPDATE orders SET is_deleted = TRUE WHERE id = $1 AND farm_id = $2 AND is_deleted = FALSE",
            id, farm_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows == 0 {
            return Err(AppError::NotFound(format!("Order {} not found", id)));
        }
        Ok(())
    }

    // ── Analytics ─────────────────────────────────────────────────────────────

    pub async fn total_revenue(&self, farm_id: Uuid, from: &str, to: &str) -> AppResult<f64> {
        let val = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(total_price), 0)
            FROM   orders
            WHERE  farm_id    = $1
              AND  is_deleted = FALSE
              AND  status     = 'DELIVERED'
              AND  ($2 = '' OR created_at::date >= $2::date)
              AND  ($3 = '' OR created_at::date <= $3::date)
            "#,
            farm_id, from, to
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(BigDecimal::from(0));

        Ok(bigdecimal_to_f64(&val))
    }

    pub async fn total_orders(&self, farm_id: Uuid, from: &str, to: &str) -> AppResult<i64> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM orders
            WHERE  farm_id    = $1
              AND  is_deleted = FALSE
              AND  ($2 = '' OR created_at::date >= $2::date)
              AND  ($3 = '' OR created_at::date <= $3::date)
            "#,
            farm_id, from, to
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        Ok(count)
    }

    pub async fn orders_by_status(&self, farm_id: Uuid, from: &str, to: &str) -> AppResult<Vec<StatusCount>> {
        let rows = sqlx::query_as!(
            StatusCount,
            r#"
            SELECT status, COUNT(*) AS "count!"
            FROM   orders
            WHERE  farm_id = $1 AND is_deleted = FALSE
            AND  ($2 = '' OR created_at::date >= $2::date)
            AND  ($3 = '' OR created_at::date <= $3::date)
            GROUP  BY status
            ORDER  BY status
            "#,
            farm_id,
            from,
            to
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn revenue_by_month(
        &self,
        farm_id: Uuid,
        from: &str,
        to: &str,
    ) -> AppResult<Vec<(String, f64, i64)>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                TO_CHAR(created_at, 'YYYY-MM') AS "month!",
                COALESCE(SUM(total_price), 0)  AS "revenue!",
                COUNT(*)                        AS "orders!"
            FROM   orders
            WHERE  farm_id    = $1
              AND  is_deleted = FALSE
              AND  status     = 'DELIVERED'
              AND  ($2 = '' OR created_at::date >= $2::date)
              AND  ($3 = '' OR created_at::date <= $3::date)
            GROUP  BY TO_CHAR(created_at, 'YYYY-MM')
            ORDER  BY 1 ASC
            "#,
            farm_id, from, to
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| (
            r.month,
            bigdecimal_to_f64(&r.revenue),
            r.orders,
        )).collect())
    }
}

// ── Helper ────────────────────────────────────────────────────────────────────

pub fn bigdecimal_to_f64(v: &BigDecimal) -> f64 {
    use std::str::FromStr;
    f64::from_str(&v.to_string()).unwrap_or(0.0)
}
