use worker::*;

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};

use crate::model::general::Status;
use crate::model::session::{AuthUser, LoginResponse, MeResponse};
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

pub async fn me(env: &Env, req: &Request) -> MeResponse {
    let cookie_header = match req.headers().get("Cookie") {
        Ok(Some(value)) => value,
        _ => {
            return MeResponse {
                authenticated: false,
                email: None,
            }
        }
    };

    let raw_token = match get_cookie_value(&cookie_header, "session") {
        Some(token) => token,
        None => {
            return MeResponse {
                authenticated: false,
                email: None,
            }
        }
    };

    let email = match sessions::get_email_with_token(env, raw_token).await {
        Ok(email) => email,
        Err(e) => {
            console_log!("Error getting email with token: {}", e);
            return MeResponse {
                authenticated: false,
                email: None,
            };
        }
    };

    MeResponse {
        authenticated: true,
        email: Some(email),
    }
}

fn get_cookie_value(cookie_header: &str, name: &str) -> Option<String> {
    cookie_header.split(';').find_map(|part| {
        let trimmed = part.trim();
        let mut pieces = trimmed.splitn(2, '=');
        let key = pieces.next()?;
        let value = pieces.next()?;

        if key == name {
            Some(value.to_string())
        } else {
            None
        }
    })
}

pub async fn require_auth(env: &Env, req: &Request) -> std::result::Result<AuthUser, Response> {
    let cookie_header = match req.headers().get("Cookie") {
        Ok(Some(value)) => value,
        _ => return Err(Response::error("Unauthorized", 401).unwrap()),
    };

    let raw_token = match get_cookie_value(&cookie_header, "session") {
        Some(token) => token,
        None => return Err(Response::error("Unauthorized", 401).unwrap()),
    };

    match sessions::get_auth_user_with_token(env, &raw_token).await {
        Ok(Some(auth_user)) => Ok(auth_user),
        Ok(None) => Err(Response::error("Unauthorized", 401).unwrap()),
        Err(e) => {
            console_log!("require_auth error: {:?}", e);
            Err(Response::error("Unauthorized", 401).unwrap())
        }
    }
}
