use worker::*;

use crate::model::user::User;

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
