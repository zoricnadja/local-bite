use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppResult;
use crate::dtos::analytics::analytics_response::TopProduct;
use crate::dtos::order_item::new_order_item_dto::NewOrderItem;
use crate::models::order_item::OrderItem;

#[derive(Clone)]
pub struct OrderItemRepository {
    pool: PgPool,
}

impl OrderItemRepository{
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_order_id(&self, order_id: Uuid) -> AppResult<Vec<OrderItem>> {
        let items = sqlx::query_as!(
            OrderItem,
            r#"
            SELECT id, order_id, product_id, product_name, product_type,
                   unit_price, quantity, unit, subtotal
            FROM   order_items
            WHERE  order_id = $1
            ORDER  BY id
            "#,
            order_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    pub async fn insert_batch(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        order_id: Uuid,
        items: &[&NewOrderItem],
    ) -> AppResult<Vec<OrderItem>> {
        let mut inserted = Vec::with_capacity(items.len());

        for item in items {
            let row = sqlx::query_as!(
                OrderItem,
                r#"
                INSERT INTO order_items
                    (id, order_id, product_id, product_name, product_type,
                     unit_price, quantity, unit)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id, order_id, product_id, product_name, product_type,
                          unit_price, quantity, unit, subtotal
                "#,
                Uuid::new_v4(), order_id,
                item.product_id,
                item.product_name,
                item.product_type,
                item.unit_price,
                item.quantity,
                item.unit
            )
            .fetch_one(&mut **tx)
            .await?;

            inserted.push(row);
        }

        Ok(inserted)
    }

    pub async fn top_products(
        &self,
        farm_id: Uuid,
        limit: i64,
        from: &str,
        to: &str,
    ) -> AppResult<Vec<TopProduct>> {
        let rows = sqlx::query_as!(
            TopProduct,
            r#"
            SELECT
                oi.product_id                    AS "product_id!",
                oi.product_name                  AS "product_name!",
                CAST(SUM(oi.quantity) AS FLOAT8)  AS "total_sold!",
                CAST(SUM(oi.subtotal) AS FLOAT8)  AS "total_revenue!"
            FROM   order_items oi
            JOIN   orders o ON o.id = oi.order_id
            WHERE  o.farm_id    = $1
              AND  o.is_deleted = FALSE
              AND  o.status     = 'DELIVERED'
              AND  ($2 = '' OR o.created_at::date >= $2::date)
              AND  ($3 = '' OR o.created_at::date <= $3::date)
            GROUP  BY oi.product_id, oi.product_name
            ORDER  BY SUM(oi.quantity) DESC
            LIMIT  $4
            "#,
            farm_id, from, to, limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
