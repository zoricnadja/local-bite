use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RawMaterialApiData {
    pub(crate) id:            Uuid,
    pub(crate) name:          String,
    pub(crate) material_type: String,
    pub(crate) unit:          String,
    pub(crate) origin:        Option<String>,
    pub(crate) supplier:      Option<String>,
}

#[derive(serde::Deserialize)]
pub struct RawMaterialWrapper {
    pub(crate) data: RawMaterialApiData,
}