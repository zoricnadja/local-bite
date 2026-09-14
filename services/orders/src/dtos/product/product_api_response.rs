use crate::dtos::product::product_api_data::ProductApiData;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct ProductApiResponse {
    pub(crate) data: ProductApiData,
}
