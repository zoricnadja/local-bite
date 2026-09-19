//! A separate durable subscription: completing a batch creates one storage lot.
use common::events::{self, IntegrationEvent};
use futures::StreamExt;
use lapin::{options::*, types::FieldTable};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{dtos::create_product_request::CreateProductRequest, services::product_policy::ProductPolicyFactory};

pub fn start(pool: PgPool) {
    tokio::spawn(async move {
        loop {
            if let Err(_error) = consume(&pool).await { tracing::warn!("Production output consumer reconnecting"); }
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });
}

async fn apply(pool: &PgPool, event: &IntegrationEvent) -> anyhow::Result<()> {
    let d=&event.data;
    if event.source!="productions" || event.entity_type!="production_batches" || event.schema_version!=1
        { return Ok(()); }
    let farm:Uuid=serde_json::from_value(d["farm_id"].clone())?;
    let mut tx=pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,0))").bind(event.entity_id.to_string()).execute(&mut *tx).await?;
    let previous:Option<i64>=sqlx::query_scalar("SELECT sequence FROM production_states WHERE batch_id=$1").bind(event.entity_id).fetch_optional(&mut *tx).await?;
    if previous.is_some_and(|n|n>=event.sequence){tx.commit().await?;return Ok(());}
    sqlx::query("INSERT INTO production_states(batch_id,farm_id,status,sequence,deleted) VALUES($1,$2,$3,$4,$5) ON CONFLICT(batch_id) DO UPDATE SET status=EXCLUDED.status,sequence=EXCLUDED.sequence,deleted=EXCLUDED.deleted WHERE production_states.sequence<EXCLUDED.sequence")
        .bind(event.entity_id).bind(farm).bind(d["status"].as_str().unwrap_or("UNKNOWN")).bind(event.sequence).bind(event.operation=="DELETE" || d["is_deleted"]==true).execute(&mut *tx).await?;
    if let Some(outputs) = d["outputs"].as_array().filter(|rows| !rows.is_empty()) {
        let cancelled = event.operation=="DELETE" || d["is_deleted"]==true || d["status"]=="CANCELLED";
        let completed = d["status"]=="COMPLETED";
        let mut keep = Vec::<Uuid>::new();
        for output in outputs {
            let output_id:Uuid=serde_json::from_value(output["id"].clone())?;
            // Product identity is local; never trust an output ID as a product ID.
            let id:Option<Uuid>=sqlx::query_scalar("SELECT product_id FROM production_outputs WHERE batch_id=$1 AND output_id=$2").bind(event.entity_id).bind(output_id).fetch_optional(&mut *tx).await?;
            let id=id.unwrap_or_else(Uuid::new_v4);
            keep.push(id);
            let expiry:Option<chrono::NaiveDate>=match output["expiry_date"].as_str().filter(|s|!s.is_empty()) {Some(s)=>Some(s.parse()?),None=>None};
            sqlx::query("INSERT INTO products(id,farm_id,name,product_type,quantity,unit,price,batch_id,qr_token,is_active,expiry_date,status,description,is_deleted) VALUES($1,$2,$3,$4,$5::float8::numeric,$6,$7::float8::numeric,$8,$9,FALSE,$10,$11,$12,$13) ON CONFLICT(id) DO UPDATE SET name=EXCLUDED.name,product_type=EXCLUDED.product_type,quantity=EXCLUDED.quantity,unit=EXCLUDED.unit,price=EXCLUDED.price,expiry_date=EXCLUDED.expiry_date,status=EXCLUDED.status,description=EXCLUDED.description,is_deleted=EXCLUDED.is_deleted WHERE products.status='PRODUCTION'")
                .bind(id).bind(farm).bind(output["name"].as_str().unwrap_or_default()).bind(output["product_type"].as_str().unwrap_or_default()).bind(output["quantity"].as_f64().unwrap_or_default()).bind(output["unit"].as_str().unwrap_or_default()).bind(output["price"].as_f64().unwrap_or_default()).bind(event.entity_id).bind(Uuid::new_v4()).bind(expiry).bind(if completed {"STORAGE"}else{"PRODUCTION"}).bind(output["description"].as_str()).bind(cancelled).execute(&mut *tx).await?;
            sqlx::query("INSERT INTO production_outputs(batch_id,product_id,output_id) VALUES($1,$2,$3) ON CONFLICT(product_id) DO NOTHING").bind(event.entity_id).bind(id).bind(output_id).execute(&mut *tx).await?;
        }
        sqlx::query("UPDATE products SET is_deleted=TRUE WHERE batch_id=$1 AND status='PRODUCTION' AND NOT(id=ANY($2))").bind(event.entity_id).bind(&keep).execute(&mut *tx).await?;
        tx.commit().await?; return Ok(());
    }
    sqlx::query("UPDATE products SET is_deleted=TRUE WHERE batch_id=$1 AND status='PRODUCTION'").bind(event.entity_id).execute(&mut *tx).await?;
    if event.operation=="DELETE" || d["status"]!="COMPLETED" || d["is_deleted"]==true { tx.commit().await?; return Ok(()); }
    // Legacy completed batches have no measured output; do not invent stock.
    let Some(quantity)=d["output_quantity"].as_f64() else { tx.commit().await?; return Ok(()); };
    anyhow::ensure!(quantity.is_finite() && quantity>0.0,"Invalid output quantity");
    let farm:Uuid=serde_json::from_value(d["farm_id"].clone())?;
    let name=d["output_name"].as_str().ok_or_else(||anyhow::anyhow!("Missing output name"))?;
    let kind=d["output_type"].as_str().ok_or_else(||anyhow::anyhow!("Missing output type"))?;
    let unit=d["output_unit"].as_str().ok_or_else(||anyhow::anyhow!("Missing output unit"))?;
    let expiry:Option<chrono::NaiveDate>=serde_json::from_value(d["output_expiry_date"].clone())?;

    let exists:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM production_outputs WHERE batch_id=$1)").bind(event.entity_id).fetch_one(&mut *tx).await?;
    if !exists {
        let id=Uuid::new_v4();
        sqlx::query("INSERT INTO products(id,farm_id,name,product_type,quantity,unit,price,batch_id,qr_token,is_active,expiry_date) VALUES($1,$2,$3,$4,$5::float8::numeric,$6,0,$7,$8,FALSE,$9)")
            .bind(id).bind(farm).bind(name).bind(kind).bind(quantity).bind(unit).bind(event.entity_id).bind(Uuid::new_v4()).bind(expiry).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO production_outputs(batch_id,product_id) VALUES($1,$2)").bind(event.entity_id).bind(id).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(())
}

async fn consume(pool:&PgPool)->anyhow::Result<()> {
    let connection=events::connect().await?;
    let channel=connection.create_channel().await?;
    events::topology(&channel).await?;
    channel.queue_declare("local_bite.production_outputs.dead.v1",QueueDeclareOptions{durable:true,..Default::default()},FieldTable::default()).await?;
    channel.confirm_select(ConfirmSelectOptions::default()).await?;
    channel.basic_qos(10,BasicQosOptions::default()).await?;
    let mut consumer=channel.basic_consume("local_bite.production_outputs.v1","products-output",BasicConsumeOptions::default(),FieldTable::default()).await?;
    while let Some(delivery)=consumer.next().await {
        let delivery=delivery?;
        let event=serde_json::from_slice::<IntegrationEvent>(&delivery.data);
        let event=match event {
            Ok(event) if validate(&event).is_ok()=>event,
            _=>{
                let confirm=channel.basic_publish("","local_bite.production_outputs.dead.v1",BasicPublishOptions{mandatory:true,..Default::default()},&delivery.data,lapin::BasicProperties::default().with_delivery_mode(2)).await?.await?;
                anyhow::ensure!(matches!(confirm,lapin::publisher_confirm::Confirmation::Ack(None)),"Dead letter publish failed");
                delivery.ack(BasicAckOptions::default()).await?;
                continue;
            }
        };
        if let Err(_error)=apply(pool,&event).await {
            delivery.nack(BasicNackOptions{multiple:false,requeue:true}).await?;
            return Err(_error);
        }
        delivery.ack(BasicAckOptions::default()).await?;
    }
    Ok(())
}

fn validate(event:&IntegrationEvent)->anyhow::Result<()> {
    anyhow::ensure!(event.schema_version==1 && event.sequence>0 && event.source=="productions" && event.entity_type=="production_batches" && matches!(event.operation.as_str(),"INSERT"|"UPDATE"|"DELETE"),"Invalid envelope");
    let d=&event.data;
    anyhow::ensure!(serde_json::from_value::<Uuid>(d["id"].clone())?==event.entity_id,"Identity mismatch");
    serde_json::from_value::<Uuid>(d["farm_id"].clone())?;
    anyhow::ensure!(matches!(d["status"].as_str(),Some("PLANNED"|"IN_PROGRESS"|"COMPLETED"|"CANCELLED")),"Invalid status");
    if !d["outputs"].is_null() {
        let outputs=d["outputs"].as_array().ok_or_else(||anyhow::anyhow!("Invalid outputs"))?;
        let mut ids=std::collections::HashSet::new();
        for output in outputs {
            let id:Uuid=serde_json::from_value(output["id"].clone())?;
            anyhow::ensure!(ids.insert(id),"Duplicate output identity");
            let expiry=output["expiry_date"].as_str().filter(|s|!s.is_empty()).map(str::parse::<chrono::NaiveDate>).transpose()?;
            let request=CreateProductRequest {
                name:output["name"].as_str().unwrap_or_default().into(),
                product_type:output["product_type"].as_str().unwrap_or_default().into(),
                unit:output["unit"].as_str().unwrap_or_default().into(),
                quantity:output["quantity"].as_f64().ok_or_else(||anyhow::anyhow!("Invalid quantity"))?,
                price:output["price"].as_f64().ok_or_else(||anyhow::anyhow!("Invalid price"))?,
                description:None,expiry_date:expiry,batch_id:Some(event.entity_id),
            };
            anyhow::ensure!(request.quantity>0.0,"Output must be positive");
            ProductPolicyFactory::for_type(&request.product_type).validate(&request)?;
        }
    }
    if d["status"]=="COMPLETED" && !d["output_quantity"].is_null() && event.operation!="DELETE" && d["is_deleted"]!=true {
        let request=CreateProductRequest{name:d["output_name"].as_str().unwrap_or_default().into(),product_type:d["output_type"].as_str().unwrap_or_default().into(),unit:d["output_unit"].as_str().unwrap_or_default().into(),quantity:d["output_quantity"].as_f64().ok_or_else(||anyhow::anyhow!("Invalid quantity"))?,price:0.0,description:None,expiry_date:serde_json::from_value(d["output_expiry_date"].clone())?,batch_id:Some(event.entity_id)};
        anyhow::ensure!(request.quantity>0.0,"Output must be positive");
        ProductPolicyFactory::for_type(&request.product_type).validate(&request)?;
    }
    Ok(())
}

#[cfg(test)] mod tests {
 use super::*;
 #[test] fn multiple_outputs_validate_identity_quantity_and_price() {
   let id=Uuid::new_v4();
   let output=serde_json::json!({"id":Uuid::new_v4(),"name":"Cheese","product_type":"cheese","unit":"kg","quantity":2,"price":5});
   let mut second=output.clone();second["id"]=serde_json::json!(Uuid::new_v4());
   let mut event=IntegrationEvent{schema_version:1,source:"productions".into(),sequence:1,entity_type:"production_batches".into(),entity_id:id,operation:"UPDATE".into(),data:serde_json::json!({"id":id,"farm_id":Uuid::new_v4(),"status":"PLANNED","outputs":[output.clone(),second]})};
   assert!(validate(&event).is_ok());
   event.data["outputs"][1]=output.clone();assert!(validate(&event).is_err());
   event.data["outputs"]=serde_json::json!([output.clone()]);
   event.data["outputs"][0]["quantity"]=serde_json::json!(0);assert!(validate(&event).is_err());
   event.data["outputs"][0]=output;
   event.data["outputs"][0]["price"]=serde_json::json!(-1);assert!(validate(&event).is_err());
 }
 #[test] fn poison_output_is_rejected(){
   let id=Uuid::new_v4();let mut event=IntegrationEvent{schema_version:1,source:"productions".into(),sequence:1,entity_type:"production_batches".into(),entity_id:id,operation:"UPDATE".into(),data:serde_json::json!({"id":id,"farm_id":Uuid::new_v4(),"status":"COMPLETED","output_name":"Cheese","output_type":"cheese","output_unit":"kg","output_quantity":2})};
   assert!(validate(&event).is_ok());event.data["output_unit"]=serde_json::json!("");assert!(validate(&event).is_err());
 }
}
