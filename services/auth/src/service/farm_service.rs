use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use argon2::{Argon2, password_hash::{SaltString, PasswordHasher}};
use jsonwebtoken::{encode, EncodingKey, Header};

use common::errors::AppError;
use common::jwt::Claims;

use crate::dtos::add_worker_request::AddWorkerRequest;
use crate::dtos::create_farm_request::CreateFarmRequest;
use crate::dtos::create_farm_response::CreateFarmResult;
use crate::dtos::update_farm_request::UpdateFarmRequest;
use crate::dtos::worker_dto::WorkerOut;
use crate::models::farms::Farm;
use crate::repository::farm_repository::FarmRepository;
use crate::repository::farm_repository::WorkerRecord;
use crate::repository::repository::UserRepository;

#[derive(Clone)]
pub struct FarmService {
    pub farm_repository: Arc<FarmRepository>,
    pub user_repository: Arc<UserRepository>,
    pub jwt_secret: String,
}

impl FarmService {
    pub fn new(farm_repository: Arc<FarmRepository>, user_repository: Arc<UserRepository>, jwt_secret: String) -> Self {
        Self { farm_repository, user_repository, jwt_secret }
    }

    pub async fn create_farm(&self, claims: &Claims, payload: CreateFarmRequest) -> Result<CreateFarmResult, AppError> {
        if claims.role != "FARM_OWNER" {
            return Err(AppError::Forbidden("Only FARM_OWNER can create a farm".into()));
        }
        if claims.farm_id.is_some() {
            return Err(AppError::Conflict("You already have a farm".into()));
        }

        let farm_id = Uuid::new_v4();
        let farm_name = payload.name.clone();
        let now = Utc::now();

        let farm = Farm {
            id: farm_id,
            name: farm_name.clone(),
            owner_id: claims.sub,
            address: payload.address,
            photo_url: payload.photo_url,
            phone: payload.phone,
            description: payload.description,
            website: payload.website,
            created_at: now,
            updated_at: now,
        };

        self.farm_repository.insert_farm(&farm).await?;
        self.user_repository.set_farm_id(claims.sub, Some(farm_id)).await?;


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

        if self.farm_repository.email_exists(&payload.email).await? {
            return Err(AppError::Conflict("Email already registered".into()));
        }

        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::default()
            .hash_password(payload.password.as_bytes(), &salt)
            .map_err(|e| AppError::BadRequest(e.to_string()))?
            .to_string();

        let worker_id = Uuid::new_v4();
        self.farm_repository
            .insert_worker(worker_id, &payload.email, &password_hash, farm_id)
            .await?;

        Ok(WorkerOut { id: worker_id, email: payload.email, role: "WORKER".to_string(), farm_id })
    }

    pub async fn get_farm(&self, claims: &Claims, farm_id: Uuid) -> Result<Farm, AppError> {
        let farm = self
            .farm_repository
            .find_by_id(farm_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Farm not found".into()))?;
        // Only allow accessing own farm for now (could be expanded to SYSTEM_ADMIN, etc.)
        if claims.sub.ne(&farm.owner_id) {
            return Err(AppError::Forbidden("You can only access your own farm".into()));
        }
        Ok(farm)
    }

    pub async fn list_farms(&self, caller: &Claims) -> Result<Vec<Farm>, AppError> {
        self.require_admin(caller)?;
        self.farm_repository.find_all().await
    }
    
    pub async fn update_farm(
        &self,
        caller: &Claims,
        farm_id: Uuid,
        payload: UpdateFarmRequest,
    ) -> Result<Farm, AppError> {
        let mut farm = self.farm_repository
            .find_by_id(farm_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Farm not found".into()))?;

        if farm.owner_id != caller.sub {
            self.require_admin(caller)?;
        }

        if let Some(v) = payload.name        { farm.name        = v; }
        if let Some(v) = payload.address     { farm.address     = v; }
        if let Some(v) = payload.photo_url   { farm.photo_url   = v; }
        if let Some(v) = payload.phone       { farm.phone       = Some(v); }
        if let Some(v) = payload.description { farm.description = Some(v); }
        if let Some(v) = payload.website     { farm.website     = Some(v); }

        farm.updated_at = Utc::now();
        self.farm_repository.update_farm(&farm).await
    }

    pub async fn delete_farm(&self, caller: &Claims, farm_id: Uuid) -> Result<(), AppError> {
        let farm = self.farm_repository
            .find_by_id(farm_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Farm not found".into()))?;
        tracing::info!("Deleting farm {}", farm.id);
        if farm.owner_id != caller.sub {
            self.require_admin(caller)?;
            tracing::info!("Removed farmdddddd {}", farm.id);
        }
        tracing::info!("Removed farm {}", farm.id);
        self.user_repository.set_farm_id(caller.sub, None).await.ok(); // best-effort
        self.farm_repository.delete_farm(farm_id).await?;
        // Detach farm from owner
        tracing::info!("Deleted farm {}", farm.id);
        Ok(())
    }
    pub async fn list_workers(&self, claims: &Claims, farm_id: Uuid) -> Result<Vec<WorkerOut>, AppError> {
        if claims.farm_id != Some(farm_id) {
            return Err(AppError::Forbidden("You can only access your own farm".into()));
        }
        let rows: Vec<WorkerRecord> = self.farm_repository.list_workers_by_farm(farm_id).await?;
        let workers = rows
            .into_iter()
            .map(|r| WorkerOut { id: r.id, email: r.email, role: "WORKER".to_string(), farm_id: r.farm_id })
            .collect();
        Ok(workers)
    }

    fn require_admin(&self, claims: &Claims) -> Result<(), AppError> {
        if claims.role != "ADMIN" {
            return Err(AppError::Forbidden("Admin access required".into()));
        }
        Ok(())
    }
}


