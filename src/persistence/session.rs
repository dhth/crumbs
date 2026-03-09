use crate::domain::{Session, SessionState, SessionToSave};
use sqlx::{Pool, Sqlite};

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
            created_at,
            updated_at
        FROM sessions
        ORDER BY updated_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(sessions)
}
