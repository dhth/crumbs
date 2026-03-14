use crate::domain::{Session, SessionState, SessionToSave};
use sqlx::{Pool, Sqlite};

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum ArchiveSessionError {
    #[error("session with id '{0}' does not exist")]
    SessionDoesNotExist(i64),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

pub async fn create_session(
    pool: &Pool<Sqlite>,
    session: SessionToSave,
) -> Result<i64, sqlx::Error> {
    let SessionToSave {
        agent_name,
        title,
        project_name,
        path,
        branch,
        state,
        timestamp,
    } = session;

    let agent_name = agent_name.as_str();
    let title = title.as_str();
    let path = path.to_string_lossy().into_owned();

    let result = sqlx::query!(
        r#"
        INSERT INTO sessions (
            agent_name,
            title,
            project_name,
            path,
            branch,
            state,
            created_at,
            updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        agent_name,
        title,
        project_name,
        path,
        branch,
        state,
        timestamp,
        timestamp,
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_sessions(pool: &Pool<Sqlite>) -> Result<Vec<Session>, sqlx::Error> {
    let sessions = sqlx::query_as!(
        Session,
        r#"
        SELECT
            id,
            agent_name,
            title,
            project_name,
            path,
            branch,
            state as "state: SessionState",
            archived_at,
            created_at,
            updated_at
        FROM sessions
        WHERE archived_at IS NULL
        ORDER BY updated_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(sessions)
}

#[allow(dead_code)]
pub async fn archive_session(
    pool: &Pool<Sqlite>,
    session_id: i64,
    archived_at: i64,
) -> Result<bool, ArchiveSessionError> {
    let result = sqlx::query!(
        r#"
        UPDATE sessions
        SET archived_at = ?
        WHERE id = ? AND archived_at IS NULL
        "#,
        archived_at,
        session_id,
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 1 {
        return Ok(true);
    }

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
        return Err(ArchiveSessionError::SessionDoesNotExist(session_id));
    }

    Ok(false)
}
