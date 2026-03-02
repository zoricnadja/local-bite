use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct Order {
    pub id:             Uuid,
    pub farm_id:        Uuid,
    pub customer_id:    Option<Uuid>,
    pub customer_name:  Option<String>,
    pub customer_email: Option<String>,
    pub status:         String,
    pub total_price:    BigDecimal,
    pub notes:          Option<String>,
    pub is_deleted:     bool,
    pub created_at:     NaiveDateTime,
    pub updated_at:     NaiveDateTime,
}
