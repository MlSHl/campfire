use worker::*;

use crate::model::User;

pub async fn insert_new_user(env: &Env, user: User) -> Result<()> {
    let db = env.d1("DB")?;

    let stmt = db.prepare(
        r#"
        INSERT INTO users (id, user_id, name, content, created_at, updated_at, deleted_at)
                   VALUES(?, ?, ?, ?, ?, ?, ?);
        "#,
    );
    let stmt = stmt.bind(&[
        log.id.into(),
        log.user_id.into(),
        log.name.into(),
        log.content.into(),
        log.created_at.into(),
        log.updated_at.into(),
        log.deleted_at.into(),
    ])?;

    stmt.run().await?;
    Ok(())
}
