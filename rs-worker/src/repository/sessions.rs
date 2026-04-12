use worker::*;

use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::model::session::Session;
use crate::service::auth::SESSION_TTL_SECONDS;

// Private functions
fn generate_session_token() -> String {
    Uuid::new_v4().to_string()
}

fn hash_session_token(raw_token: &str) -> String {
    let digest = Sha256::digest(raw_token.as_bytes());
    hex::encode(digest)
}

async fn insert_new_session(env: &Env, session: Session) -> Result<()> {
    let db = env.d1("DB")?;

    let stmt = db.prepare(
        "insert into sessions (id, user_id, token_hash, created_at, expires_at) 
            values(?1, ?2, ?3, ?4, ?5)",
    );
    let stmt = stmt.bind(&[
        session.id.into(),
        session.user_id.into(),
        session.token_hash.into(),
        session.created_at.into(),
        session.expires_at.into(),
    ])?;

    stmt.run().await?;

    Ok(())
}

// Public functions
pub async fn get_password_hash(env: &Env, email: &str) -> Result<String> {
    let db = env.d1("DB")?;

    let stmt = db.prepare("select password_hash from users where email = ?");
    let stmt = stmt.bind(&[email.into()])?;

    let result = stmt.first::<String>(None).await?;

    match result {
        Some(hash) => Ok(hash),
        None => Err(worker::Error::RustError("user not found".into())),
    }
}

pub async fn create_new_session(env: &Env, user_id: &str) -> Result<String> {
    let raw_token = generate_session_token();
    let token_hash = hash_session_token(&raw_token);
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now();
    let expires_at = created_at + Duration::seconds(SESSION_TTL_SECONDS);

    let session = Session {
        id,
        user_id: user_id.to_string(),
        token_hash,
        created_at: created_at.to_rfc3339(),
        expires_at: expires_at.to_rfc3339(),
    };

    insert_new_session(env, session).await?;

    Ok(raw_token)
}
