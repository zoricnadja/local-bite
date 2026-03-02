use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use common::errors::{AppError, AppResult};
use common::paginated_response::PaginatedResponse;
use crate::{
    repositories::{
        order_item_repository::OrderItemRepository,
        order_repository::{bigdecimal_to_f64, OrderRepository},
    },
};
use crate::dtos::analytics::analytics_response::{AnalyticsResponse, MonthlyRevenue};
use crate::dtos::order::create_order_request::CreateOrderRequest;
use crate::dtos::order::list_orders_query::ListOrdersQuery;
use crate::dtos::order::order_response::OrderResponse;
use crate::dtos::order::update_status_request::UpdateStatusRequest;
use crate::dtos::order_item::new_order_item_dto::NewOrderItem;
use crate::dtos::order_item::order_item_response::OrderItemResponse;
use crate::models::order::Order;
use crate::models::order_item::OrderItem;
use crate::models::order_status::OrderStatus;
use crate::services::product_service::fetch_product;

#[derive(Clone)]
pub struct OrderService {
    order_repository: Arc<OrderRepository>,
    order_item_repository: Arc<OrderItemRepository>,
}

impl OrderService {
    pub fn new(order_repository: Arc<OrderRepository>, order_item_repository: Arc<OrderItemRepository>) -> Self {
        Self { order_repository, order_item_repository }
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
            let items = self.order_item_repository.find_by_order_id(order.id).await?;
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

    pub async fn get_order(
        &self,
        id: Uuid,
        farm_id: Uuid,
    ) -> AppResult<OrderResponse> {
        let order = self.order_repository.find_by_id(id, farm_id).await?;
        let items = self.order_item_repository.find_by_order_id(order.id).await?;
        Ok(map_order_response(order, items))
    }

    // ── Create order ──────────────────────────────────────────────────────────

    pub async fn create_order(
        &self,
        farm_id: Uuid,
        req: CreateOrderRequest,
        token: &str,
    ) -> AppResult<OrderResponse> {
        // Validate items
        if req.items.is_empty() {
            return Err(AppError::BadRequest("Order must have at least one item".into()));
        }
        for item in &req.items {
            if item.quantity <= 0.0 {
                return Err(AppError::BadRequest(
                    format!("quantity must be > 0 for product {}", item.product_id),
                ));
            }
        }

        // Validate all products concurrently before touching DB
        let product_futures: Vec<_> = req
            .items
            .iter()
            .map(|item| fetch_product(item.product_id, token))
            .collect();

        let snapshots = futures_join_all(product_futures).await?;

        // Check all products are active
        for snap in &snapshots {
            if !snap.is_active {
                return Err(AppError::BadRequest(
                    format!("Product '{}' is not currently available", snap.name),
                ));
            }
        }

        // Build new items with denormalized product data
        let new_items: Vec<NewOrderItem> = req
            .items
            .iter()
            .zip(snapshots.iter())
            .map(|(item, snap)| NewOrderItem {
                product_id:   snap.id,
                product_name: snap.name.clone(),
                product_type: snap.product_type.clone(),
                unit_price:   BigDecimal::from_str(&snap.price.to_string()).unwrap_or_default(),
                quantity:     BigDecimal::from_str(&item.quantity.to_string()).unwrap_or_default(),
                unit:         snap.unit.clone(),
            })
            .collect();

        // Calculate total
        let total_price: BigDecimal = new_items
            .iter()
            .map(|i| &i.unit_price * &i.quantity)
            .fold(BigDecimal::from(0), |acc, x| acc + x);

        let mut tx = self.order_repository.pool.begin().await?;

        let order = self.order_repository
            .insert(
                &mut tx,
                farm_id,
                req.customer_id,
                req.customer_name.as_deref(),
                req.customer_email.as_deref(),
                req.notes.as_deref(),
                &total_price,
            )
            .await?;

        let items = self.order_item_repository
            .insert_batch(&mut tx, order.id, &new_items)
            .await?;

        tx.commit().await?;

        Ok(map_order_response(order, items))
    }

    // ── Update status ─────────────────────────────────────────────────────────

    pub async fn update_status(
        &self,
        id: Uuid,
        farm_id: Uuid,
        req: UpdateStatusRequest,
        caller_role: &str,
    ) -> AppResult<OrderResponse> {
        let current = self.order_repository.find_by_id(id, farm_id).await?;

        let current_status = OrderStatus::from_str(&current.status)
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Unknown current status")))?;

        let next_status = OrderStatus::from_str(&req.status)
            .ok_or_else(|| AppError::BadRequest(format!("Unknown status '{}'", req.status)))?;

        // Only FARM_OWNER can cancel
        if next_status == OrderStatus::Cancelled && caller_role != "FARM_OWNER" {
            return Err(AppError::Forbidden("Only FARM_OWNER can cancel orders".into()));
        }

        if !current_status.can_transition_to(&next_status) {
            return Err(AppError::BadRequest(format!(
                "Invalid status transition: {} → {}",
                current_status.as_str(),
                next_status.as_str()
            )));
        }

        let updated = self.order_repository
            .update_status(id, farm_id, next_status.as_str())
            .await?;

        let items = self.order_item_repository
            .find_by_order_id(updated.id)
            .await?;

        Ok(map_order_response(updated, items))
    }

    // ── Cancel order ──────────────────────────────────────────────────────────

    pub async fn cancel_order(&self, id: Uuid, farm_id: Uuid) -> AppResult<OrderResponse> {
        self.update_status(
            id,
            farm_id,
            UpdateStatusRequest { status: "CANCELLED".into() },
            "FARM_OWNER",
        )
        .await
    }

    // ── Delete order ──────────────────────────────────────────────────────────

    pub async fn delete_order(&self, id: Uuid, farm_id: Uuid) -> AppResult<()> {
        let order = self.order_repository.find_by_id(id, farm_id).await?;

        // Only PENDING or CANCELLED orders can be deleted
        if !matches!(order.status.as_str(), "PENDING" | "CANCELLED") {
            return Err(AppError::BadRequest(
                "Only PENDING or CANCELLED orders can be deleted".into(),
            ));
        }

        self.order_repository
            .soft_delete(id, farm_id)
            .await
    }

    // ── Analytics ─────────────────────────────────────────────────────────────

    pub async fn get_analytics(
        &self,
        farm_id: Uuid,
        from: &str,
        to: &str,
    ) -> AppResult<AnalyticsResponse> {

        let (total_revenue, total_orders, orders_by_status, monthly_raw, top_products) =
            tokio::try_join!(
                self.order_repository.total_revenue(farm_id, from, to),
                 self.order_repository.total_orders(farm_id, from, to),
                 self.order_repository.orders_by_status(farm_id),
                 self.order_repository.revenue_by_month(farm_id, from, to),
                 self.order_item_repository.top_products(farm_id, 10, from, to),
            )?;

        let revenue_by_month = monthly_raw
            .into_iter()
            .map(|(month, revenue, orders)| MonthlyRevenue { month, revenue, orders })
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

fn map_order_response(order: Order, items: Vec<OrderItem>) -> OrderResponse {
    OrderResponse {
        id:             order.id,
        farm_id:        order.farm_id,
        customer_id:    order.customer_id,
        customer_name:  order.customer_name,
        customer_email: order.customer_email,
        status:         order.status,
        total_price:    bigdecimal_to_f64(&order.total_price),
        notes:          order.notes,
        created_at:     order.created_at,
        updated_at:     order.updated_at,
        items: items
            .into_iter()
            .map(|i| OrderItemResponse {
                id:           i.id,
                product_id:   i.product_id,
                product_name: i.product_name,
                product_type: i.product_type,
                unit_price:   bigdecimal_to_f64(&i.unit_price),
                quantity:     bigdecimal_to_f64(&i.quantity),
                unit:         i.unit,
                subtotal:     bigdecimal_to_f64(&i.subtotal),
            })
            .collect(),
    }
}

/// Runs all futures concurrently and collects results,
/// returning the first error if any validation fails.
async fn futures_join_all<T>(
    futures: Vec<impl std::future::Future<Output = anyhow::Result<T>>>,
) -> AppResult<Vec<T>> {
    let results = futures::future::join_all(futures).await;
    let mut out = Vec::with_capacity(results.len());
    for r in results {
        out.push(r.map_err(|e| AppError::BadRequest(e.to_string()))?);
    }
    Ok(out)
}
