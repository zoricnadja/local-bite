//! Durable checkout: persist intent before reserving; retry the same reservation after crashes.
use std::{collections::BTreeMap, str::FromStr, sync::Arc};
use bigdecimal::BigDecimal;
use common::{errors::{AppError,AppResult}, service_auth};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use sqlx::Row;
use uuid::Uuid;
use crate::{services::{order_service::{OrderService,map_order_response},product_service::fetch_product},dtos::{order::create_order_request::CreateOrderRequest,order_item::new_order_item_dto::NewOrderItem}};

#[derive(Serialize,Deserialize)]
struct Line { product_id:Uuid, farm_id:Uuid, name:String, kind:String, unit:String, quantity:String, unit_price:String }

pub async fn submit(service:&OrderService,id:Uuid,customer:Uuid,email:&str,req:CreateOrderRequest,token:&str)->AppResult<Value>{
    let request=json!({"items":req.items,"notes":req.notes});
    if let Some(row)=sqlx::query("SELECT customer_id,request FROM checkout_jobs WHERE id=$1").bind(id).fetch_optional(&service.order_repository.pool).await? {
        if row.get::<Uuid,_>("customer_id")!=customer || row.get::<Value,_>("request")!=request {return Err(AppError::Conflict("Idempotency key already used for another request".into()));}
        return process(service,id).await;
    }
    if req.items.is_empty() {return Err(AppError::BadRequest("Order requires items".into()));}
    let mut quantities:BTreeMap<Uuid,BigDecimal>=BTreeMap::new();
    for item in &req.items {
        if !item.quantity.is_finite() || item.quantity<=0.0 {return Err(AppError::BadRequest("Quantity must be positive".into()));}
        let q=BigDecimal::from_str(&item.quantity.to_string()).map_err(|e|AppError::Internal(e.into()))?;
        if q!=q.with_scale(3){return Err(AppError::BadRequest("Quantity supports at most three decimals".into()));}
        *quantities.entry(item.product_id).or_insert_with(||BigDecimal::from(0))+=q;
    }
    let mut lines=Vec::new();
    for (product_id,quantity) in quantities {
        let p=fetch_product(product_id,token).await.map_err(|_|AppError::BadRequest("Product unavailable".into()))?;
        if !p.is_active{return Err(AppError::BadRequest("Product is not on sale".into()));}
        lines.push(Line{product_id,farm_id:p.farm_id.ok_or_else(||AppError::BadRequest("Product has no producer".into()))?,name:p.name,kind:p.product_type,unit:p.unit,quantity:quantity.to_string(),unit_price:p.price.to_string()});
    }
    let payload=json!({"lines":lines,"customer_name":req.customer_name,"customer_email":email,"notes":req.notes});
    sqlx::query("INSERT INTO checkout_jobs(id,customer_id,request,payload) VALUES($1,$2,$3,$4) ON CONFLICT DO NOTHING")
        .bind(id).bind(customer).bind(&request).bind(payload).execute(&service.order_repository.pool).await?;
    // A concurrent request may have claimed the key during validation.
    let owner=sqlx::query("SELECT customer_id,request FROM checkout_jobs WHERE id=$1").bind(id).fetch_one(&service.order_repository.pool).await?;
    if owner.get::<Uuid,_>("customer_id")!=customer || owner.get::<Value,_>("request")!=request{return Err(AppError::Conflict("Idempotency key already used".into()));}
    process(service,id).await
}

async fn process(service:&OrderService,id:Uuid)->AppResult<Value>{
    let mut tx=service.order_repository.pool.begin().await?;
    let row=sqlx::query("SELECT customer_id,payload,status,response,error FROM checkout_jobs WHERE id=$1 FOR UPDATE").bind(id).fetch_one(&mut *tx).await?;
    let status:String=row.get("status");
    if status=="COMPLETED" {return Ok(row.get("response"));}
    if status=="FAILED" {return Err(AppError::Conflict(row.get::<Option<String>,_>("error").unwrap_or_else(||"Checkout failed".into())));}
    let payload:Value=row.get("payload");
    let lines:Vec<Line>=serde_json::from_value(payload["lines"].clone()).map_err(|e|AppError::Internal(e.into()))?;
    let items:Vec<Value>=lines.iter().map(|l|json!({"product_id":l.product_id,"quantity":l.quantity,"unit_price":l.unit_price})).collect();
    let base=std::env::var("PRODUCTS_SERVICE_URL").unwrap_or_else(|_|"http://products-service:3003".into());
    let result=reqwest::Client::new().post(format!("{base}/internal/reservations/{id}"))
        .bearer_auth(service_auth::token("ORDER_STOCK",id,None)?).json(&items).timeout(std::time::Duration::from_secs(10)).send().await;
    let response=match result {Ok(r)=>r,Err(_)=>return Err(AppError::Conflict("Checkout is pending; retry with the same Idempotency-Key".into()))};
    if !response.status().is_success(){
        if matches!(response.status().as_u16(),400|409){
            sqlx::query("UPDATE checkout_jobs SET status='FAILED',error='Product unavailable, price changed or insufficient stock',updated_at=now() WHERE id=$1").bind(id).execute(&mut *tx).await?;
            tx.commit().await?;
            return Err(AppError::Conflict("Product unavailable, price changed or insufficient stock".into()));
        }
        return Err(AppError::Conflict("Checkout is pending; retry with the same Idempotency-Key".into()));
    }
    let mut farms:BTreeMap<Uuid,Vec<NewOrderItem>>=BTreeMap::new();
    for l in lines {
        farms.entry(l.farm_id).or_default().push(NewOrderItem{product_id:l.product_id,product_name:l.name,product_type:l.kind,farm_id:l.farm_id,unit:l.unit,
            quantity:BigDecimal::from_str(&l.quantity).map_err(|e|AppError::Internal(e.into()))?,unit_price:BigDecimal::from_str(&l.unit_price).map_err(|e|AppError::Internal(e.into()))?});
    }
    let mut orders=Vec::new();
    for (farm,items) in farms {
        let total=items.iter().map(|l|(&l.quantity*&l.unit_price).round(2)).fold(BigDecimal::from(0),|a,b|a+b);
        let order=service.order_repository.insert(&mut tx,farm,row.get("customer_id"),payload["customer_name"].as_str(),payload["customer_email"].as_str().unwrap_or_default().into(),payload["notes"].as_str(),&total).await?;
        let saved=service.order_item_repository.insert_batch(&mut tx,order.id,&items.iter().collect::<Vec<_>>()).await?;
        sqlx::query("INSERT INTO order_stock_links(order_id,checkout_id,farm_id) VALUES($1,$2,$3)").bind(order.id).bind(id).bind(farm).execute(&mut *tx).await?;
        orders.push(map_order_response(order,saved));
    }
    let response=json!({"orders":orders});
    sqlx::query("UPDATE checkout_jobs SET status='COMPLETED',response=$2,updated_at=now() WHERE id=$1").bind(id).bind(&response).execute(&mut *tx).await?;
    tx.commit().await?;Ok(response)
}

pub async fn release_pending(service:&OrderService)->AppResult<()> {
    let jobs:Vec<(Uuid,Uuid,Uuid)>=sqlx::query_as("SELECT order_id,checkout_id,farm_id FROM stock_release_jobs WHERE completed_at IS NULL ORDER BY created_at LIMIT 50").fetch_all(&service.order_repository.pool).await?;
    let base=std::env::var("PRODUCTS_SERVICE_URL").unwrap_or_else(|_|"http://products-service:3003".into());
    for (order,id,farm) in jobs {
        let result=reqwest::Client::new().post(format!("{base}/internal/reservations/{id}/release/{farm}"))
            .bearer_auth(service_auth::token("ORDER_STOCK",id,Some(farm))?).timeout(std::time::Duration::from_secs(10)).send().await;
        if result.is_ok_and(|r|r.status().is_success()) {
            sqlx::query("UPDATE stock_release_jobs SET completed_at=now() WHERE order_id=$1").bind(order).execute(&service.order_repository.pool).await?;
        }
    }
    Ok(())
}

pub fn start(service:Arc<OrderService>){tokio::spawn(async move{loop{
    if let Ok(ids)=sqlx::query_scalar::<_,Uuid>("SELECT id FROM checkout_jobs WHERE status='PENDING' ORDER BY created_at LIMIT 20").fetch_all(&service.order_repository.pool).await{
        for id in ids {if process(&service,id).await.is_err(){tracing::warn!(%id,"Checkout pending or rejected; inspect workflow status");}}
    }
    if release_pending(&service).await.is_err(){tracing::warn!("Stock release retry pending");}
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
}});}
