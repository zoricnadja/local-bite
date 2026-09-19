


use std::sync::Arc;
use uuid::Uuid;

use crate::dtos::analytics::analytics_response::{AnalyticsResponse, MonthlyRevenue};
use crate::dtos::order::create_order_request::CreateOrderRequest;

use crate::dtos::order::list_orders_query::ListOrdersQuery;
use crate::dtos::order::order_response::OrderResponse;
use crate::dtos::order::update_status_request::UpdateStatusRequest;

use crate::dtos::order_item::order_item_response::OrderItemResponse;
use crate::models::order::Order;
use crate::models::order_item::OrderItem;
use crate::models::order_status::OrderStatus;
use crate::repositories::{
    order_item_repository::OrderItemRepository,
    order_repository::{bigdecimal_to_f64, OrderRepository},
};

use common::errors::{AppError, AppResult};
use common::paginated_response::PaginatedResponse;

#[derive(Clone)]
pub struct OrderService {
    pub(crate) order_repository: Arc<OrderRepository>,
    pub(crate) order_item_repository: Arc<OrderItemRepository>,
}

impl OrderService {
    pub fn new(
        order_repository: Arc<OrderRepository>,
        order_item_repository: Arc<OrderItemRepository>,
    ) -> Self {
        Self {
            order_repository,
            order_item_repository,
        }
    }

    // ── List orders ───────────────────────────────────────────────────────────

    pub async fn list_orders(
        &self,
        farm_id: Uuid,
        q: &ListOrdersQuery,
    ) -> AppResult<PaginatedResponse<OrderResponse>> {
        let (orders, total) = tokio::try_join!(
            self.order_repository.find_all(farm_id, q),
            self.order_repository.count(farm_id, q),
        )?;

        let mut responses = Vec::with_capacity(orders.len());
        for order in orders {
            let items = self
                .order_item_repository
                .find_by_order_id(order.id)
                .await?;
            responses.push(map_order_response(order, items));
        }

        Ok(PaginatedResponse {
            data: responses,
            total,
            page: q.page.unwrap_or(1),
            limit: q.limit(),
        })
    }

    pub async fn find_all_by_user_id(
        &self,
        _id: Uuid,
        q: &ListOrdersQuery,
    ) -> AppResult<PaginatedResponse<OrderResponse>> {
        let (orders, total) = tokio::try_join!(
            self.order_repository.find_all_by_user_id(_id, q),
            self.order_repository.count_by_user(_id, q),
        )?;

        let mut responses = Vec::with_capacity(orders.len());
        for order in orders {
            let items = self
                .order_item_repository
                .find_by_order_id(order.id)
                .await?;
            responses.push(map_order_response(order, items));
        }

        Ok(PaginatedResponse {
            data: responses,
            total,
            page: q.page.unwrap_or(1),
            limit: q.limit(),
        })
    }

    // ── Get single order ──────────────────────────────────────────────────────

    pub async fn get_order(&self, id: Uuid, claims: &common::jwt::Claims) -> AppResult<OrderResponse> {
        let order = self.order_repository.find_by_id(id).await?;
        let allowed = match claims.role.as_str() {
            "SYSTEM_ADMIN" => true,
            "CUSTOMER" => order.customer_id == Some(claims.sub),
            "FARM_OWNER" | "WORKER" => claims.farm_id == Some(order.farm_id),
            _ => false,
        };
        if !allowed { return Err(AppError::NotFound("Order not found".into())); }
        let items = self
            .order_item_repository
            .find_by_order_id(order.id)
            .await?;
        Ok(map_order_response(order, items))
    }

    // ── Create order ──────────────────────────────────────────────────────────

    pub async fn create_order(&self, id: Uuid, email: &str, req: CreateOrderRequest, token: &str, key: Uuid) -> AppResult<serde_json::Value> {
        crate::checkout::submit(self,key,id,email,req,token).await
    }
    // ── Update status ─────────────────────────────────────────────────────────

    pub async fn update_status(
        &self,
        id: Uuid,
        farm_id: Uuid,
        req: UpdateStatusRequest,
        caller_role: &str,
    ) -> AppResult<OrderResponse> {
        let mut tx = self.order_repository.pool.begin().await?;
        let current = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id=$1 AND farm_id=$2 AND NOT is_deleted FOR UPDATE")
            .bind(id).bind(farm_id).fetch_optional(&mut *tx).await?.ok_or_else(||AppError::NotFound("Order not found".into()))?;

        let current_status = OrderStatus::from_str(&current.status)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Unknown current status")))?;

        let next_status = OrderStatus::from_str(&req.status)
            .ok_or_else(|| AppError::BadRequest(format!("Unknown status '{}'", req.status)))?;

        // Only FARM_OWNER can cancel
        if next_status == OrderStatus::Cancelled && caller_role != "FARM_OWNER" {
            return Err(AppError::Forbidden(
                "Only FARM_OWNER can cancel orders".into(),
            ));
        }

        if !current_status.can_transition_to(&next_status) {
            return Err(AppError::BadRequest(format!(
                "Invalid status transition: {} → {}",
                current_status.as_str(),
                next_status.as_str()
            )));
        }

        let updated = sqlx::query_as::<_,Order>("UPDATE orders SET status=$3 WHERE id=$1 AND farm_id=$2 RETURNING *")
            .bind(id).bind(farm_id).bind(next_status.as_str()).fetch_one(&mut *tx).await?;
        if next_status == OrderStatus::Cancelled {
            sqlx::query("INSERT INTO stock_release_jobs(order_id,checkout_id,farm_id) SELECT order_id,checkout_id,farm_id FROM order_stock_links WHERE order_id=$1 ON CONFLICT DO NOTHING").bind(id).execute(&mut *tx).await?;
        }
        tx.commit().await?;
        let _ = crate::checkout::release_pending(self).await;
        let items = self
            .order_item_repository
            .find_by_order_id(updated.id)
            .await?;

        Ok(map_order_response(updated, items))
    }

    // ── Cancel order ──────────────────────────────────────────────────────────



    // ── Delete order ──────────────────────────────────────────────────────────

    pub async fn delete_order(&self, id: Uuid, farm_id: Uuid) -> AppResult<()> {
        let mut tx=self.order_repository.pool.begin().await?;
        let order=sqlx::query_as::<_,Order>("SELECT * FROM orders WHERE id=$1 AND farm_id=$2 AND NOT is_deleted FOR UPDATE").bind(id).bind(farm_id).fetch_optional(&mut *tx).await?.ok_or_else(||AppError::NotFound("Order not found".into()))?;
        if !matches!(order.status.as_str(),"PENDING"|"CANCELLED"){return Err(AppError::BadRequest("Only pending or cancelled orders may be deleted".into()));}
        sqlx::query("INSERT INTO stock_release_jobs(order_id,checkout_id,farm_id) SELECT order_id,checkout_id,farm_id FROM order_stock_links WHERE order_id=$1 ON CONFLICT DO NOTHING").bind(id).execute(&mut *tx).await?;
        sqlx::query("UPDATE orders SET is_deleted=true,status='CANCELLED' WHERE id=$1").bind(id).execute(&mut *tx).await?;
        tx.commit().await?;
        let _=crate::checkout::release_pending(self).await;
        Ok(())
    }

    // ── Analytics ─────────────────────────────────────────────────────────────

    pub async fn get_analytics(
        &self,
        farm_id: Uuid,
        from: &str,
        to: &str,
    ) -> AppResult<AnalyticsResponse> {
        let (total_revenue, total_orders, orders_by_status, monthly_raw, top_products) = tokio::try_join!(
            self.order_repository.total_revenue(farm_id, from, to),
            self.order_repository.total_orders(farm_id, from, to),
            self.order_repository.orders_by_status(farm_id, from, to),
            self.order_repository.revenue_by_month(farm_id, from, to),
            self.order_item_repository
                .top_products(farm_id, 10, from, to),
        )?;

        let revenue_by_month = monthly_raw
            .into_iter()
            .map(|(month, revenue, orders)| MonthlyRevenue {
                month,
                revenue,
                orders,
            })
            .collect();

        Ok(AnalyticsResponse {
            total_revenue,
            total_orders,
            orders_by_status,
            revenue_by_month,
            top_products,
        })
    }
}

// ── Mapping helpers ───────────────────────────────────────────────────────────

pub(crate) fn map_order_response(order: Order, items: Vec<OrderItem>) -> OrderResponse {
    OrderResponse {
        id: order.id,
        farm_id: order.farm_id,
        customer_id: order.customer_id,
        customer_name: order.customer_name,
        customer_email: order.customer_email,
        status: order.status,
        total_price: bigdecimal_to_f64(&order.total_price),
        notes: order.notes,
        created_at: order.created_at,
        updated_at: order.updated_at,
        items: items
            .into_iter()
            .map(|i| OrderItemResponse {
                id: i.id,
                product_id: i.product_id,
                product_name: i.product_name,
                product_type: i.product_type,
                unit_price: bigdecimal_to_f64(&i.unit_price),
                quantity: bigdecimal_to_f64(&i.quantity),
                unit: i.unit,
                subtotal: bigdecimal_to_f64(&i.subtotal),
            })
            .collect(),
    }
}
