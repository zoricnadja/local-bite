use crate::repository::repository::UserRepository;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use common::{errors::AppError, jwt::Claims};
use crate::dtos::update_user_request::UpdateUserRequest;
use crate::models::user::User;

#[derive(Clone)]
pub struct UserService {
    pub user_repo: Arc<UserRepository>,
    pub jwt_secret: String,
}

impl UserService {
    pub fn new(user_repo: Arc<UserRepository>, jwt_secret: String) -> Self {
        Self { user_repo, jwt_secret }
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<User, AppError> {
        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))
    }

    pub async fn list_users(&self, claims: &Claims) -> Result<Vec<User>, AppError> {
        self.require_admin(claims)?;
        self.user_repo.find_all().await
    }

    pub async fn update_user(
        &self,
        caller: &Claims,
        target_id: Uuid,
        payload: UpdateUserRequest,
    ) -> Result<User, AppError> {
        // Allow self-update or admin
        if caller.sub != target_id {
            self.require_admin(caller)?;
        }

        let mut user = self.get_user(target_id).await?;

        if let Some(email) = payload.email {
            // Ensure new email isn't taken by someone else
            if let Some(existing) = self.user_repo.find_by_email(&email).await? {
                if existing.id != target_id {
                    return Err(AppError::Conflict("Email already in use".into()));
                }
            }
            user.email = email;
        }

        if let Some(password) = payload.password {
            let salt = SaltString::generate(rand::thread_rng());
            user.password_hash = Argon2::default()
                .hash_password(password.as_bytes(), &salt)?
                .to_string();
        }

        if let Some(role_str) = payload.role {
            self.require_admin(caller)?;
            user.role = role_str.parse()?;
        }

        if let Some(v) = payload.first_name { user.first_name = v; }
        if let Some(v) = payload.last_name  { user.last_name  = v; }
        if let Some(v) = payload.address    { user.address    = v; }
        if let Some(v) = payload.phone      { user.phone      = Some(v); }
        if let Some(v) = payload.photo_url  { user.photo_url  = Some(v); }
        if let Some(v) = payload.date_of_birth { user.date_of_birth = Some(v); }

        user.updated_at = Utc::now();
        self.user_repo.update_user(&user).await
    }

    pub async fn delete_user(&self, target_id: Uuid) -> Result<(), AppError> {
        self.user_repo
            .find_by_id(target_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;
        self.user_repo.delete_user(target_id).await
    }

    fn require_admin(&self, claims: &Claims) -> Result<(), AppError> {
        if claims.role != "ADMIN" {
            return Err(AppError::Forbidden("Admin access required".into()));
        }
        Ok(())
    }

}