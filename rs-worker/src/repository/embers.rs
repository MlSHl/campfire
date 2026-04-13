use worker::*;

use crate::model::ember::{ClientEmber, Ember};

pub async fn get_all_by_user(env: &Env, user_id: &str) -> Result<Vec<Ember>> {
    let db = env.d1("DB")?;

    let stmt = db.prepare("SELECT * FROM embers WHERE user_id = ?1");
    let stmt = stmt.bind(&[user_id.into()])?;

    let result = stmt.all().await?;
    let embers: Vec<Ember> = result.results()?;

    Ok(embers)
}

pub async fn insert(env: &Env, user_id: &str, ember: &ClientEmber) -> Result<()> {
    let db = env.d1("DB")?;

    let stmt = db.prepare(
        r#"
        INSERT INTO embers (
            id,
            user_id,
            name,
            hours_today,
            hours_this_week,
            hours_total,
            created_at,
            updated_at,
            deleted_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#,
    );

    let stmt = stmt.bind(&[
        ember.id.clone().into(),
        user_id.into(),
        ember.name.clone().into(),
        ember.hours_today.into(),
        ember.hours_this_week.into(),
        ember.hours_total.into(),
        ember.created_at.clone().into(),
        ember.updated_at.clone().into(),
        ember.deleted_at.clone().into(),
    ])?;

    stmt.run().await?;
    Ok(())
}

pub async fn update(env: &Env, user_id: &str, ember: &ClientEmber) -> Result<()> {
    let db = env.d1("DB")?;

    let stmt = db.prepare(
        r#"
        UPDATE embers
        SET
            name = ?1,
            hours_today = ?2,
            hours_this_week = ?3,
            hours_total = ?4,
            created_at = ?5,
            updated_at = ?6,
            deleted_at = ?7
        WHERE id = ?8 AND user_id = ?9
        "#,
    );

    let stmt = stmt.bind(&[
        ember.name.clone().into(),
        ember.hours_today.into(),
        ember.hours_this_week.into(),
        ember.hours_total.into(),
        ember.created_at.clone().into(),
        ember.updated_at.clone().into(),
        ember.deleted_at.clone().into(),
        ember.id.clone().into(),
        user_id.into(),
    ])?;

    stmt.run().await?;
    Ok(())
}
