use worker::*;

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
