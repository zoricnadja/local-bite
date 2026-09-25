use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;
use uuid::Uuid;


use crate::dtos::create_business_request::CreateBusinessRequest;
use crate::dtos::create_business_response::CreateBusinessResult;
use crate::dtos::register_request::RegisterRequest;
use crate::dtos::update_business_request::UpdateBusinessRequest;
use crate::dtos::worker_dto::WorkerOut;
use crate::models::businesses::Business;
use crate::models::user::User;
use crate::repository::business_repository::BusinessRepository;
use crate::repository::business_repository::WorkerRecord;
use crate::repository::repository::UserRepository;
use common::errors::AppError;
use common::jwt::Claims;
use common::models::Role;

#[derive(Clone)]
pub struct BusinessService {
    pub business_repository: Arc<BusinessRepository>,
    pub user_repository: Arc<UserRepository>,
    pub jwt_secret: String,
}

impl BusinessService {
    pub fn new(
        business_repository: Arc<BusinessRepository>,
        user_repository: Arc<UserRepository>,
        jwt_secret: String,
    ) -> Self {
        Self {
            business_repository,
            user_repository,
            jwt_secret,
        }
    }

    pub async fn create_business(
        &self,
        claims: &Claims,
        payload: CreateBusinessRequest,
    ) -> Result<CreateBusinessResult, AppError> {
        if claims.role != "BUSINESS_OWNER" {
            return Err(AppError::Forbidden(
                "Only business owners can add a business".into(),
            ));
        }
        if self.business_repository.find_by_owner(claims.sub).await?.is_some() {
            return Err(AppError::Conflict("You already have a business".into()));
        }

        let business_id = Uuid::new_v4();
        let business_name = payload.name.clone();
        let now = Utc::now();

        let business = Business {
            id: business_id,
            name: business_name.clone(),
            owner_id: claims.sub,
            address: payload.address,
            phone: payload.phone,
            description: payload.description,
            website: payload.website,
            created_at: now,
            updated_at: now,
        };

        self.business_repository.insert_business(&business).await?;


        // Issue a fresh JWT including the new business_id
        let new_claims = Claims {
            sub: claims.sub,
            email: claims.email.clone(),
            role: claims.role.clone(),
            business_id: Some(business_id),
            exp: (Utc::now().timestamp() + 3600) as usize,
            iat: Utc::now().timestamp() as usize,
        };
        let token = encode(
            &Header::default(),
            &new_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(CreateBusinessResult { business, token })
    }

    pub async fn add_worker(
        &self,
        claims: &Claims,
        business_id: Uuid,
        payload: RegisterRequest,
    ) -> Result<WorkerOut, AppError> {
        if claims.role != "BUSINESS_OWNER" {
            return Err(AppError::Forbidden(
                "Only BUSINESS_OWNER can add workers".into(),
            ));
        }
        if self.business_repository.find_by_id(business_id).await?.map(|f| f.owner_id) != Some(claims.sub) {
            return Err(AppError::Forbidden(
                "You can only add workers to your own business".into(),
            ));
        }

        if self.business_repository.email_exists(&payload.email).await? {
            return Err(AppError::Conflict("Email already registered".into()));
        }

        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::default()
            .hash_password(payload.password.as_bytes(), &salt)
            .map_err(|e| AppError::BadRequest(e.to_string()))?
            .to_string();
        let now = Utc::now();

        let worker_id = Uuid::new_v4();
        let user = User {
            id: worker_id,
            email: payload.email.clone(),
            password_hash,
            business_id: Some(business_id),
            role: Role::Worker,
            first_name: payload.first_name,
            last_name: payload.last_name,
            address: payload.address,
            phone: payload.phone,
            photo_url: payload.photo_url,
            date_of_birth: payload.date_of_birth,
            created_at: now,
            updated_at: now,
        };
        self.user_repository.create_user(user).await?;

        Ok(WorkerOut {
            id: worker_id,
            email: payload.email,
            role: "WORKER".to_string(),
            business_id,
        })
    }

    pub async fn get_business(&self, claims: &Claims, business_id: Uuid) -> Result<Business, AppError> {
        let business = self
            .business_repository
            .find_by_id(business_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Business not found".into()))?;
        let user = self.user_repository.find_by_id(claims.sub).await?
            .ok_or_else(|| AppError::Unauthorized("Account not found".into()))?;
        if user.id != business.owner_id && !(user.role == Role::Worker && user.business_id == Some(business_id)) {
            return Err(AppError::Forbidden(
                "You can only access your own business".into(),
            ));
        }
        Ok(business)
    }

    pub async fn list_businesses(&self, caller: &Claims) -> Result<Vec<Business>, AppError> {
        self.require_admin(caller)?;
        self.business_repository.find_all().await
    }

    pub async fn update_business(
        &self,
        caller: &Claims,
        business_id: Uuid,
        payload: UpdateBusinessRequest,
    ) -> Result<Business, AppError> {
        let mut business = self
            .business_repository
            .find_by_id(business_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Business not found".into()))?;

        if business.owner_id != caller.sub {
            self.require_admin(caller)?;
        }

        if let Some(v) = payload.name {
            business.name = v;
        }
        if let Some(v) = payload.address {
            business.address = v;
        }
        if let Some(v) = payload.phone {
            business.phone = Some(v);
        }
        if let Some(v) = payload.description {
            business.description = Some(v);
        }
        if let Some(v) = payload.website {
            business.website = Some(v);
        }

        business.updated_at = Utc::now();
        self.business_repository.update_business(&business).await
    }

    pub async fn delete_business(&self, caller: &Claims, business_id: Uuid) -> Result<(), AppError> {
        let business = self
            .business_repository
            .find_by_id(business_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Business not found".into()))?;
        if business.owner_id != caller.sub {
            self.require_admin(caller)?;
            }

        self.business_repository.delete_business(business_id).await?;
        // Detach business from owner
        Ok(())
    }
    pub async fn list_workers(
        &self,
        claims: &Claims,
        business_id: Uuid,
    ) -> Result<Vec<WorkerOut>, AppError> {
        if claims.role != "BUSINESS_OWNER" || self.business_repository.find_by_id(business_id).await?.map(|f| f.owner_id) != Some(claims.sub) {
            return Err(AppError::Forbidden(
                "You can only access your own business".into(),
            ));
        }
        let rows: Vec<WorkerRecord> = self.business_repository.list_workers_by_business(business_id).await?;
        let workers = rows
            .into_iter()
            .map(|r| WorkerOut {
                id: r.id,
                email: r.email,
                role: "WORKER".to_string(),
                business_id: r.business_id,
            })
            .collect();
        Ok(workers)
    }

    fn require_admin(&self, claims: &Claims) -> Result<(), AppError> {
        if claims.role != "SYSTEM_ADMIN" {
            return Err(AppError::Forbidden("Admin access required".into()));
        }
        Ok(())
    }
}
