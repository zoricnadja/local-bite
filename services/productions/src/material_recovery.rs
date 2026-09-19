//! Durable compensation journal. Advisory locks distinguish in-flight local writes from crashes.
use sqlx::PgPool;
use uuid::Uuid;
pub async fn journal(pool:&PgPool,tx:&mut sqlx::Transaction<'_,sqlx::Postgres>,id:Uuid,farm:Uuid)->common::errors::AppResult<()> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,22))").bind(id.to_string()).execute(&mut **tx).await?;
    // Separate committed record survives rollback of the business transaction.
    sqlx::query("INSERT INTO material_intents(operation_id,farm_id) VALUES($1,$2) ON CONFLICT DO NOTHING").bind(id).bind(farm).execute(pool).await?;
    Ok(())
}
async fn release(farm:Uuid,id:Uuid)->anyhow::Result<()> {
    let base=std::env::var("RAW_MATERIALS_SERVICE_URL").unwrap_or_else(|_|"http://raw-materials-service:3002".into());
    reqwest::Client::new().post(format!("{base}/internal/production-consumption/release"))
        .bearer_auth(common::service_auth::token("MATERIAL_STOCK",Uuid::nil(),Some(farm))?).json(&vec![id])
        .timeout(std::time::Duration::from_secs(10)).send().await?.error_for_status()?;
    Ok(())
}
pub async fn recover(pool:&PgPool)->anyhow::Result<()> {
    let pending:Vec<(Uuid,Uuid)>=sqlx::query_as("SELECT operation_id,farm_id FROM material_intents WHERE completed_at IS NULL AND created_at<now()-interval '30 seconds' ORDER BY created_at LIMIT 50").fetch_all(pool).await?;
    for (id,farm) in pending {
        let mut tx=pool.begin().await?;
        let locked:bool=sqlx::query_scalar("SELECT pg_try_advisory_xact_lock(hashtextextended($1,22))").bind(id.to_string()).fetch_one(&mut *tx).await?;
        if !locked {continue;}
        let exists:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM batch_raw_materials WHERE id=$1)").bind(id).fetch_one(&mut *tx).await?;
        if !exists {release(farm,id).await?;}
        sqlx::query("UPDATE material_intents SET completed_at=now() WHERE operation_id=$1").bind(id).execute(&mut *tx).await?;
        tx.commit().await?;
    }
    let jobs:Vec<(Uuid,Uuid)>=sqlx::query_as("SELECT operation_id,farm_id FROM material_release_jobs WHERE completed_at IS NULL ORDER BY created_at LIMIT 50").fetch_all(pool).await?;
    for (id,farm) in jobs {
        release(farm,id).await?;
        sqlx::query("UPDATE material_release_jobs SET completed_at=now() WHERE operation_id=$1").bind(id).execute(pool).await?;
    }
    Ok(())
}
pub fn start(pool:PgPool){tokio::spawn(async move{loop{
    if recover(&pool).await.is_err(){tracing::warn!("Material compensation pending; retrying");}
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
}});}
