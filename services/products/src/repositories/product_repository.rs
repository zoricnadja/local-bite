
use sqlx::PgPool;
use uuid::Uuid;


use crate::models::product::Product;
use crate::models::query::ListQuery;
use crate::models::update_product_params::UpdateParams;
use common::errors::{AppError, AppResult};

#[derive(Clone)]
pub struct ProductRepository {
    pub pool: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ── List ──────────────────────────────────────────────────────────────────────

    pub async fn find_all_by_farm_id(
        &self,
        farm_id: Uuid,
        q: &ListQuery,
    ) -> AppResult<(Vec<Product>, i64)> {
        self.list_filtered(Some(farm_id), q).await
    }
    pub async fn find_all(&self, q: &ListQuery) -> AppResult<(Vec<Product>, i64)> {
        self.list_filtered(q.farm_id,q).await
    }
    async fn list_filtered(&self, farm:Option<Uuid>, q:&ListQuery)->AppResult<(Vec<Product>,i64)> {
        let active=q.is_active.or_else(|| if q.active_only==Some(true) {Some(true)} else {None});
        let available="(p.is_active AND (p.expiry_date IS NULL OR p.expiry_date>=CURRENT_DATE) AND EXISTS(SELECT 1 FROM production_states b WHERE b.batch_id=p.batch_id AND b.farm_id=p.farm_id AND b.status='COMPLETED' AND NOT b.deleted))";
        let filter=format!("FROM products p WHERE NOT p.is_deleted AND ($1::uuid IS NULL OR p.farm_id=$1) AND ($2='' OR p.product_type ILIKE $2) AND ($3='' OR p.name ILIKE '%'||$3||'%') AND ($4::bool IS NULL OR {}=$4) AND ($7='' OR p.status=$7)",available);
        let select=format!("SELECT p.id,p.farm_id,p.name,p.product_type,p.description,p.quantity,p.unit,p.price,p.expiry_date,p.batch_id,p.image_path,p.qr_token,p.qr_path,p.status,{} AS is_active,p.is_deleted,p.created_at,p.updated_at {} ORDER BY p.created_at DESC,p.id LIMIT $5 OFFSET $6",available,filter);
        let items=sqlx::query_as::<_,Product>(&select).bind(farm).bind(q.product_type.as_deref().unwrap_or("")).bind(q.search.as_deref().unwrap_or("")).bind(active).bind(q.limit()).bind(q.offset()).bind(q.status.as_deref().unwrap_or("")).fetch_all(&self.pool).await?;
        let total=sqlx::query_scalar::<_,i64>(&format!("SELECT count(*) {}",filter.replace("$7", "$5"))).bind(farm).bind(q.product_type.as_deref().unwrap_or("")).bind(q.search.as_deref().unwrap_or("")).bind(active).bind(q.status.as_deref().unwrap_or("")).fetch_one(&self.pool).await?;
        Ok((items,total))
    }

    // ── Find ──────────────────────────────────────────────────────────────────────

    pub async fn find_by_id_and_farm(&self, id: Uuid, farm_id: Uuid) -> AppResult<Product> {
        sqlx::query_as::<_, Product>(r#"
            SELECT id, farm_id, name, product_type, description, quantity, unit, price,
                   expiry_date, batch_id, image_path, qr_token, qr_path, status, is_active, is_deleted,
                   created_at, updated_at
            FROM   products
            WHERE  id = $1 AND farm_id = $2 AND is_deleted = FALSE
            "#).bind(id).bind(farm_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Product {} not found", id)))
    }

    pub async fn find_by_id(&self, id: Uuid) -> AppResult<Product> {
        sqlx::query_as::<_, Product>(r#"
            SELECT id, farm_id, name, product_type, description, quantity, unit, price,
                   expiry_date, batch_id, image_path, qr_token, qr_path, status, is_active, is_deleted,
                   created_at, updated_at
            FROM   products
            WHERE  id = $1 AND is_deleted = FALSE
            "#).bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Product {} not found", id)))
    }

    pub async fn find_by_qr_token(&self, qr_token: Uuid) -> AppResult<Product> {
        sqlx::query_as::<_, Product>(r#"
            SELECT id, farm_id, name, product_type, description, quantity, unit, price,
                   expiry_date, batch_id, image_path, qr_token, qr_path, status, is_active, is_deleted,
                   created_at, updated_at
            FROM   products
            WHERE  qr_token  = $1
              AND  is_active = TRUE
              AND  is_deleted = FALSE
            "#).bind(qr_token)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Product not found or no longer active".into()))
    }

    // ── Insert ────────────────────────────────────────────────────────────────────



    // ── Update ────────────────────────────────────────────────────────────────────
    pub async fn update(&self, id: Uuid, farm_id: Uuid, p: UpdateParams) -> AppResult<Product> {
        Ok(sqlx::query_as::<_, Product>(r#"
            UPDATE products SET name=$1, product_type=$2, description=$3,
                quantity=CASE WHEN EXISTS(SELECT 1 FROM production_outputs WHERE product_id=products.id)
                    THEN quantity ELSE COALESCE($4,quantity) END,
                unit=$5, price=$6, batch_id=$7, is_active=$8, expiry_date=$11, status=$12
            WHERE id=$9 AND farm_id=$10 AND NOT is_deleted RETURNING *
        "#).bind(p.name).bind(p.product_type).bind(p.description).bind(p.quantity)
        .bind(p.unit).bind(p.price).bind(p.batch_id).bind(p.is_active).bind(id).bind(farm_id)
        .bind(p.expiry_date).bind(p.status).fetch_one(&self.pool).await?)
    }

    pub async fn soft_delete(&self, id: Uuid, farm_id: Uuid) -> AppResult<u64> {
        Ok(sqlx::query!(
            "UPDATE products SET is_deleted = TRUE WHERE id = $1 AND farm_id = $2 AND is_deleted = FALSE",
            id, farm_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected())
    }

    // ── QR / Image ────────────────────────────────────────────────────────────────

    pub async fn set_qr_path(&self, id: Uuid, qr_path: &str) -> AppResult<()> {
        sqlx::query!(
            "UPDATE products SET qr_path = $1 WHERE id = $2",
            qr_path,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn set_qr_path_returning(
        &self,
        id: Uuid,
        farm_id: Uuid,
        qr_path: &str,
    ) -> AppResult<Product> {
        Ok(sqlx::query_as::<_, Product>(r#"
            UPDATE products SET qr_path = $1 WHERE id = $2 AND farm_id = $3
            RETURNING id, farm_id, name, product_type, description, quantity, unit, price,
                      expiry_date, batch_id, image_path, qr_token, qr_path, status, is_active, is_deleted,
                      created_at, updated_at
            "#).bind(qr_path).bind(id).bind(farm_id)
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn set_image_path(
        &self,
        id: Uuid,
        farm_id: Uuid,
        image_path: &str,
    ) -> AppResult<Product> {
        Ok(sqlx::query_as::<_, Product>(r#"
            UPDATE products SET image_path = $1
            WHERE id = $2 AND farm_id = $3
            RETURNING id, farm_id, name, product_type, description, quantity, unit, price,
                      expiry_date, batch_id, image_path, qr_token, qr_path, status, is_active, is_deleted,
                      created_at, updated_at
            "#).bind(image_path).bind(id).bind(farm_id)
        .fetch_one(&self.pool)
        .await?)
    }


}
