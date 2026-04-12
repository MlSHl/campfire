use worker::{console_log, Env};

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};

use crate::model::general::Status;
use crate::model::session::LoginResponse;
use crate::model::user::UserDto;

use crate::repository::sessions;
use crate::repository::users;

pub const SESSION_TTL_SECONDS: i64 = 60 * 60 * 24 * 30;

fn is_valid_password(password: &str, stored_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(&stored_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub async fn login(env: &Env, user: UserDto) -> LoginResponse {
    let stored_hash = match sessions::get_password_hash(env, &user.email).await {
        Ok(hash) => hash,
        Err(e) => {
            console_log!("Error in session::auth::login: {}", e);
            return LoginResponse::new(Status::Error, "login failed", None);
        }
    };
    if !is_valid_password(&user.password, &stored_hash) {
        return LoginResponse::new(Status::Error, "invalid username or password", None);
    }

    let user_id = match users::get_user_id_by_email(env, &user.email).await {
        Ok(id) => id,
        Err(_) => {
            return LoginResponse::new(Status::Error, "invalid username or password", None);
        }
    };

    let raw_token = match sessions::create_new_session(env, &user_id).await {
        Ok(token) => token,
        Err(_) => {
            return LoginResponse::new(Status::Error, "login failed", None);
        }
    };

    LoginResponse::new(Status::Ok, "login successful", Some(raw_token))
}
