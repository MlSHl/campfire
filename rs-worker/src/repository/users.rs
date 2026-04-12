use worker::*;

use crate::model::user::{User, UserIdRow};

pub async fn insert_new_user(env: &Env, user: User) -> Result<()> {
    let db = env.d1("DB")?;

    let stmt = db.prepare(
        r#"
        INSERT INTO users (id, email, password_hash, created_at)
                   VALUES(?, ?, ?, ?);
        "#,
    );
    let stmt = stmt.bind(&[
        user.id.into(),
        user.email.into(),
        user.password_hash.into(),
        user.created_at.into(),
    ])?;

    stmt.run().await?;
    Ok(())
}

pub async fn get_user_id_by_email(env: &Env, user_email: &str) -> Result<String> {
    let db = env.d1("DB")?;

    let stmt = db.prepare("select id from users where email = ?1");
    let stmt = stmt.bind(&[user_email.into()])?;

    let user_id = stmt.first::<UserIdRow>(None).await?;

    match user_id {
        Some(row) => Ok(row.id),
        None => Err(worker::Error::RustError("user not found".to_string()).into()),
    }
}
