use serde::Deserialize;
use crate::dtos::product::product_api_data::ProductApiData;
#[derive(Deserialize)]
pub struct ProductApiResponse {
    pub(crate) data: ProductApiData,
}