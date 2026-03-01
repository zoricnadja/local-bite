use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::SaltString, PasswordHash};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;
use std::sync::Arc;
use chrono::Utc;
use common::errors::AppError;
use common::jwt::Claims;
use common::jwt::decode_jwt;
use crate::models::user::User;
use crate::repository::repository::UserRepository;
use crate::dtos::login_request::LoginRequest;
use crate::dtos::register_request::RegisterRequest;

#[derive(Clone)]
pub struct AuthService {
    pub user_repo: Arc<UserRepository>,
    pub jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repo: Arc<UserRepository>, jwt_secret: String) -> Self {
        Self { user_repo, jwt_secret }
    }

    pub async fn register_user(&self, payload: RegisterRequest) -> Result<(), AppError> {
        if self.user_repo.find_by_email(&payload.email).await?.is_some() {
            return Err(AppError::Conflict("Email already in use".into()));
        }

        let salt = SaltString::generate(rand::thread_rng());
        let password_hash = Argon2::default()
            .hash_password(payload.password.as_bytes(), &salt)?
            .to_string();

        let user = User {
            id: Uuid::new_v4(),
            email: payload.email,
            password_hash,
            role: payload.role
                .unwrap_or_else(|| "CUSTOMER".to_string())
                .parse()?,
            created_at: Utc::now().naive_utc(),
        };

        self.user_repo.create_user(user).await?;
        Ok(())
    }

    pub async fn login(&self, payload: LoginRequest) -> Result<String, AppError> {
        let user = self.user_repo
            .find_by_email(&payload.email)
            .await?
            .ok_or(AppError::Unauthorized("Invalid email or password".into()))?;

        let parsed_hash = PasswordHash::new(&user.password_hash)?;
        Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Unauthorized("Invalid email or password".into()))?;

        let claims = Claims {
            sub: user.id,
            email: user.email,
            role: String::from(user.role.as_str()),
            farm_id: None,
            exp: (Utc::now().timestamp() + 3600) as usize,
            iat: 0,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<User, AppError> {
        self.user_repo.find_by_id(user_id)
            .await?
            .ok_or(AppError::NotFound("User not found".into()))
    }

    pub fn verify_token(&self, token: &str) -> Result<Uuid, AppError> {
        let token_data = decode_jwt(token, &self.jwt_secret)
            .map_err(|_| AppError::Unauthorized("Invalid token".into()))?;
        Ok(token_data.claims.sub)
    }
}