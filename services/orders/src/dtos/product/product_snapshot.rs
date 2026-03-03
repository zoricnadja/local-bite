use uuid::Uuid;

pub struct ProductSnapshot {
    pub id:           Uuid,
    pub name:         String,
    pub product_type: String,
    pub price:        f64,
    pub unit:         String,
    pub is_active:    bool,
    pub farm_id:      Option<Uuid>,
}