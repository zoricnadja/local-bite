use crate::dtos::create_product_request::CreateProductRequest;
use common::errors::{AppError, AppResult};

/// Strategy pattern: validation can evolve per product family without turning
/// ProductService into a growing conditional block.
pub trait ProductPolicy: Send + Sync {
    fn validate(&self, request: &CreateProductRequest) -> AppResult<()>;
}

pub struct FoodProductPolicy;
impl ProductPolicy for FoodProductPolicy {
    fn validate(&self, request: &CreateProductRequest) -> AppResult<()> {
        required_fields(request)?;
        if request.unit.trim().is_empty() {
            return Err(AppError::BadRequest(
                "Food products require a unit of measure".into(),
            ));
        }
        Ok(())
    }
}
pub struct GenericProductPolicy;
impl ProductPolicy for GenericProductPolicy {
    fn validate(&self, request: &CreateProductRequest) -> AppResult<()> {
        required_fields(request)
    }
}
fn required_fields(request: &CreateProductRequest) -> AppResult<()> {
    if request.name.trim().is_empty() {
        return Err(AppError::BadRequest("Name cannot be empty".into()));
    }
    if request.product_type.trim().is_empty() {
        return Err(AppError::BadRequest("Product type cannot be empty".into()));
    }
    if !request.price.is_finite() || request.price < 0.0 {
        return Err(AppError::BadRequest("Price cannot be negative".into()));
    }
    if !request.quantity.is_finite() || request.quantity < 0.0 || request.quantity >= 1_000_000_000.0 || (request.quantity*1000.0-(request.quantity*1000.0).round()).abs()>0.000001 {
        return Err(AppError::BadRequest("Quantity cannot be negative".into()));
    }
    Ok(())
}
/// Factory pattern: adding a family only adds a policy, not controller branches.
pub struct ProductPolicyFactory;
impl ProductPolicyFactory {
    pub fn for_type(product_type: &str) -> Box<dyn ProductPolicy> {
        match product_type.trim().to_ascii_lowercase().as_str() {
            value if (value != "other" && common::product_types::is_supported_product_type(value))
                || matches!(value, "food" | "juice" | "preserve") => {
                Box::new(FoodProductPolicy)
            }
            _ => Box::new(GenericProductPolicy),
        }
    }
}
