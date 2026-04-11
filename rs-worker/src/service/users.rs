use crate::model::general::MessageResponse;
use crate::model::user::{User, UserDto};

use crate::repository::sessions::get_password_hash;
use crate::repository::users::insert_new_user;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use uuid::Uuid;
use worker::{console_error, Env};

pub async fn register_user(env: &Env, user: UserDto) -> MessageResponse {
    let user_id = Uuid::new_v4().to_string();
    let password_hash = match hash_password(&user.password) {
        Ok(hash) => hash,
        Err(e) => {
            console_error!("Hashing error: {}", e);
            return MessageResponse {
                status: "error",
                message: "password hashing failed".to_string(),
            };
        }
    };

    let created_at = Utc::now().to_rfc3339();

    let user = User {
        id: user_id,
        email: user.email,
        password_hash,
        created_at,
    };

    let result = insert_new_user(env, user).await;

    match result {
        Ok(_) => MessageResponse {
            status: "ok",
            message: "user registered successfully".to_string(),
        },
        Err(e) => {
            console_error!("Database error: {}", e);
            MessageResponse {
                status: "error",
                message: "failed to register user".to_string(),
            }
        }
    }
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(hash)
}
