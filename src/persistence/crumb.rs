use crate::domain::{Crumb, CrumbToSave, SessionState};
use sqlx::{Pool, Sqlite};

#[derive(Debug, thiserror::Error)]
pub enum AddCrumbError {
    #[error("session with id '{0}' does not exist")]
    SessionDoesNotExist(i64),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetCrumbsError {
    #[error("session with id '{0}' does not exist")]
    SessionDoesNotExist(i64),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

pub async fn add_crumb(pool: &Pool<Sqlite>, crumb: CrumbToSave) -> Result<(), AddCrumbError> {
    let CrumbToSave {
        session_id,
        message,
        state,
        confidence,
        timestamp,
    } = crumb;

    let session_id_value = session_id.get();
    let crumb_message = message.as_str();
    let confidence = confidence.map(|confidence| i64::from(confidence.get()));
    let mut tx = pool.begin().await?;

    let saved_session = sqlx::query!(
        r#"
        SELECT id
        FROM sessions
        WHERE id = ?
        "#,
        session_id_value,
    )
    .fetch_optional(&mut *tx)
    .await?;

    saved_session.ok_or(AddCrumbError::SessionDoesNotExist(session_id_value))?;

    sqlx::query!(
        r#"
        INSERT INTO crumbs (
            session_id,
            message,
            state,
            confidence,
            created_at
        ) VALUES (?, ?, ?, ?, ?)
        "#,
        session_id_value,
        crumb_message,
        state,
        confidence,
        timestamp,
    )
    .execute(&mut *tx)
    .await?;

    match state {
        Some(state) => {
            sqlx::query!(
                r#"
                UPDATE sessions
                SET state = ?, updated_at = ?
                WHERE id = ?
                "#,
                state,
                timestamp,
                session_id_value,
            )
            .execute(&mut *tx)
            .await?;
        }
        None => {
            sqlx::query!(
                r#"
                UPDATE sessions
                SET updated_at = ?
                WHERE id = ?
                "#,
                timestamp,
                session_id_value,
            )
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    Ok(())
}

pub async fn get_crumbs(
    pool: &Pool<Sqlite>,
    session_id: i64,
) -> Result<Vec<Crumb>, GetCrumbsError> {
    let session_exists = sqlx::query!(
        r#"
        SELECT id
        FROM sessions
        WHERE id = ?
        "#,
        session_id,
    )
    .fetch_optional(pool)
    .await?;

    if session_exists.is_none() {
        return Err(GetCrumbsError::SessionDoesNotExist(session_id));
    }

    let crumbs = sqlx::query_as!(
        Crumb,
        r#"
        SELECT
            id,
            session_id,
            message,
            state as "state: SessionState",
            confidence,
            created_at
        FROM crumbs
        WHERE session_id = ?
        ORDER BY created_at ASC
        "#,
        session_id,
    )
    .fetch_all(pool)
    .await?;

    Ok(crumbs)
}
