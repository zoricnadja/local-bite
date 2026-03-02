use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct AnalyticsResponse {
    pub total_revenue:       f64,
    pub total_orders:        i64,
    pub orders_by_status:    Vec<StatusCount>,
    pub revenue_by_month:    Vec<MonthlyRevenue>,
    pub top_products:        Vec<TopProduct>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StatusCount {
    pub status: String,
    pub count:  i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MonthlyRevenue {
    pub month:   String,    // "2024-03"
    pub revenue: f64,
    pub orders:  i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TopProduct {
    pub product_id:   Uuid,
    pub product_name: String,
    pub total_sold:   f64,
    pub total_revenue: f64,
}