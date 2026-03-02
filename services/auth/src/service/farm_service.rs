use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use argon2::{Argon2, password_hash::{SaltString, PasswordHasher}};
use jsonwebtoken::{encode, EncodingKey, Header};

use common::errors::AppError;
use common::jwt::Claims;

use crate::dtos::add_worker_request::AddWorkerRequest;
use crate::dtos::create_farm_request::CreateFarmRequest;
use crate::models::farms::Farm;
use crate::repository::farm_repository::FarmRepository;
use crate::repository::farm_repository::WorkerRecord;

#[derive(Clone)]
pub struct FarmService {
    pub repo: Arc<FarmRepository>,
    pub jwt_secret: String,
}

impl FarmService {
    pub fn new(repo: Arc<FarmRepository>, jwt_secret: String) -> Self { Self { repo, jwt_secret } }

    pub async fn create_farm(&self, claims: &Claims, payload: CreateFarmRequest) -> Result<CreateFarmResult, AppError> {
        if claims.role != "FARM_OWNER" {
            return Err(AppError::Forbidden("Only FARM_OWNER can create a farm".into()));
        }
        if claims.farm_id.is_some() {
            return Err(AppError::Conflict("You already have a farm".into()));
        }

        let farm_id = Uuid::new_v4();
        let mut tx = self.repo.pool.begin().await?;

        self.repo.insert_farm_tx(&mut tx, farm_id, &payload.name, claims.sub).await?;
        self.repo.set_user_farm_tx(&mut tx, claims.sub, farm_id).await?;

        tx.commit().await?;

        let farm = Farm { id: farm_id, name: payload.name, owner_id: claims.sub, created_at: Utc::now().naive_utc() };

        // Issue a fresh JWT including the new farm_id
        let new_claims = Claims {
            sub: claims.sub,
            email: claims.email.clone(),
            role: claims.role.clone(),
            farm_id: Some(farm_id),
            exp: (Utc::now().timestamp() + 3600) as usize,
            iat: Utc::now().timestamp() as usize,
        };
        let token = encode(
            &Header::default(),
            &new_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(CreateFarmResult { farm, token })
    }

    pub async fn add_worker(&self, claims: &Claims, farm_id: Uuid, payload: AddWorkerRequest) -> Result<WorkerOut, AppError> {
        if claims.role != "FARM_OWNER" {
            return Err(AppError::Forbidden("Only FARM_OWNER can add workers".into()));
        }
        if claims.farm_id != Some(farm_id) {
            return Err(AppError::Forbidden("You can only add workers to your own farm".into()));
        }

        if self.repo.email_exists(&payload.email).await? {
            return Err(AppError::Conflict("Email already registered".into()));
        }

        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::default()
            .hash_password(payload.password.as_bytes(), &salt)
            .map_err(|e| AppError::BadRequest(e.to_string()))?
            .to_string();

        let worker_id = Uuid::new_v4();
        self.repo
            .insert_worker(worker_id, &payload.email, &password_hash, farm_id)
            .await?;

        Ok(WorkerOut { id: worker_id, email: payload.email, role: "WORKER".to_string(), farm_id })
    }

    pub async fn get_farm(&self, claims: &Claims, farm_id: Uuid) -> Result<Farm, AppError> {
        // Only allow accessing own farm for now (could be expanded to SYSTEM_ADMIN, etc.)
        if claims.farm_id != Some(farm_id) {
            return Err(AppError::Forbidden("You can only access your own farm".into()));
        }
        let farm = self
            .repo
            .find_by_id(farm_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Farm not found".into()))?;
        Ok(farm)
    }

    pub async fn list_workers(&self, claims: &Claims, farm_id: Uuid) -> Result<Vec<WorkerOut>, AppError> {
        if claims.farm_id != Some(farm_id) {
            return Err(AppError::Forbidden("You can only access your own farm".into()));
        }
        let rows: Vec<WorkerRecord> = self.repo.list_workers_by_farm(farm_id).await?;
        let workers = rows
            .into_iter()
            .map(|r| WorkerOut { id: r.id, email: r.email, role: "WORKER".to_string(), farm_id: r.farm_id })
            .collect();
        Ok(workers)
    }
}

#[derive(serde::Serialize)]
pub struct CreateFarmResult {
    pub farm: Farm,
    pub token: String,
}

#[derive(serde::Serialize)]
pub struct WorkerOut {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub farm_id: Uuid,
}
