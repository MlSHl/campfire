use worker::*;

use crate::model::logbook::Log;

pub async fn get_logs_for_user(env: &Env, user_id: &str) -> Result<Vec<Log>> {
    let db = env.d1("DB")?;

    let stmt = db.prepare("SELECT * FROM logs WHERE user_id = ?");
    let stmt = stmt.bind(&[user_id.into()])?;

    let result = stmt.all().await?;
    let logs: Vec<Log> = result.results()?;
    Ok(logs)
}

pub async fn insert_log_for_user(env: &Env, log: Log) -> Result<()> {
    let db = env.d1("DB")?;

    let stmt = db.prepare(
        r#"
        INSERT INTO logs (id, user_id, name, content, created_at, updated_at, deleted_at)
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
