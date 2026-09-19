//! Short-lived tokens for internal commands, scoped to one durable operation.
use crate::{errors::{AppError, AppResult}, jwt::{Claims, encode_jwt}};
use uuid::Uuid;

pub fn token(role: &str, operation: Uuid, farm: Option<Uuid>) -> AppResult<String> {
    let now = chrono::Utc::now().timestamp() as usize;
    let secret = std::env::var("JWT_SECRET").map_err(|e| AppError::Internal(e.into()))?;
    Ok(encode_jwt(&Claims { sub: operation, email: String::new(), role: role.into(),
        farm_id: farm, iat: now, exp: now + 60 }, &secret)?)
}

pub fn require(claims: &Claims, role: &str, operation: Uuid) -> AppResult<()> {
    if claims.role != role || claims.sub != operation {
        return Err(AppError::Forbidden("Internal operation scope mismatch".into()));
    }
    Ok(())
}
