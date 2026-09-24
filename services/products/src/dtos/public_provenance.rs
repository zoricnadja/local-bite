//! Explicit public allowlist; internal identifiers, suppliers, prices and stock never cross it.
use serde::Serialize;
use super::provenance_response::ProvenanceResponse;

#[derive(Serialize)]
pub struct PublicProduct { pub name:String, pub product_type:String, pub description:Option<String>, pub expiry_date:Option<chrono::NaiveDate>, pub qr_token:uuid::Uuid }
#[derive(Serialize)]
pub struct PublicMaterial { pub name:String, pub material_type:String, pub origin:Option<String>, pub harvest_date:Option<chrono::NaiveDate> }
#[derive(Serialize)]
pub struct PublicStep { pub status:String, pub step_order:i32, pub name:String, pub description:Option<String>, pub variables:Vec<crate::models::process_step_ref::StepVariableRef> }
#[derive(Serialize)]
pub struct PublicBatch { pub name:String, pub start_date:Option<String>, pub end_date:Option<String>, pub status:String, pub steps:Vec<PublicStep>, pub raw_materials:Vec<PublicMaterial> }
#[derive(Serialize)]
pub struct PublicProvenance { pub product:PublicProduct, pub farm_name:Option<String>, pub batch:Option<PublicBatch> }

impl From<ProvenanceResponse> for PublicProvenance {
    fn from(p:ProvenanceResponse)->Self {
        Self {product:PublicProduct{name:p.product.name,product_type:p.product.product_type,description:p.product.description,expiry_date:p.product.expiry_date,qr_token:p.product.qr_token},farm_name:p.farm_name,
            batch:p.batch.map(|b|PublicBatch{name:b.name,start_date:b.start_date,end_date:b.end_date,status:b.status,
                steps:b.steps.into_iter().map(|s|PublicStep{status:s.status,step_order:s.step_order,name:s.name,description:s.description,variables:s.variables}).collect(),
                raw_materials:b.raw_materials.into_iter().map(|m|PublicMaterial{name:m.name,material_type:m.material_type,origin:m.origin,harvest_date:m.harvest_date}).collect()})}
    }
}
