use crate::config::ThemeName;
use crate::domain::{
    AgentName, Confidence, CrumbMessage, CrumbToSave, SessionId, SessionState, SessionTitle,
    SessionToSave,
};
use crate::persistence::{
    AddCrumbError, GetCrumbsError, add_crumb, create_session, get_crumbs, get_sessions,
};
use crate::tui::TuiConfig;
use crate::utils::git::current_branch;
use anyhow::Context;
use serde::Serialize;
use sqlx::{Pool, Sqlite};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
struct RegisterResponse {
    session_id: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum RegisterSessionError {
    #[error("invalid input provided: {0}")]
    InvalidInput(&'static str),
    #[error("couldn't get current working directory: {0}")]
    CouldntGetCwd(#[from] std::io::Error),
    #[error("couldn't save result in database: {0}")]
    Persistence(#[from] sqlx::Error),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum LogCrumbError {
    #[error("invalid input provided: {0}")]
    InvalidInput(&'static str),
    #[error(transparent)]
    Persistence(#[from] AddCrumbError),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ListSessionsError {
    #[error("couldn't fetch sessions from database: {0}")]
    Persistence(#[from] sqlx::Error),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ListCrumbsError {
    #[error(transparent)]
    Persistence(#[from] GetCrumbsError),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

pub async fn handle_register_session(
    pool: &Pool<Sqlite>,
    agent_name: String,
    title: String,
) -> Result<(), RegisterSessionError> {
    let agent_name = AgentName::try_from(agent_name).map_err(RegisterSessionError::InvalidInput)?;
    let title = SessionTitle::try_from(title).map_err(RegisterSessionError::InvalidInput)?;

    let path = std::env::current_dir()?;
    let project_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .context("couldn't determine project name from current directory")?;
    // TODO: log this error when tracing is set up
    let branch = current_branch(&path).unwrap_or(None);
    let timestamp = unix_timestamp_now().context("couldn't get current timestamp")?;

    let session_to_save = SessionToSave {
        agent_name,
        title,
        project_name,
        path,
        branch,
        state: SessionState::Working,
        timestamp,
    };

    let session_id = create_session(pool, session_to_save).await?;

    println!(
        "{}",
        serde_json::to_string(&RegisterResponse { session_id })
            .context("couldn't serialize response")?
    );

    Ok(())
}

pub async fn handle_log_crumb(
    pool: &Pool<Sqlite>,
    session_id: i64,
    message: String,
    state: Option<SessionState>,
    confidence: Option<u8>,
) -> Result<(), LogCrumbError> {
    let session_id = SessionId::try_from(session_id).map_err(LogCrumbError::InvalidInput)?;
    let message = CrumbMessage::try_from(message).map_err(LogCrumbError::InvalidInput)?;
    let confidence = confidence
        .map(Confidence::try_from)
        .transpose()
        .map_err(LogCrumbError::InvalidInput)?;

    let timestamp = unix_timestamp_now().context("couldn't get current timestamp")?;

    let crumb_to_save = CrumbToSave {
        session_id,
        message,
        state,
        confidence,
        timestamp,
    };

    add_crumb(pool, crumb_to_save).await?;

    Ok(())
}

pub async fn handle_list_sessions(pool: &Pool<Sqlite>) -> Result<(), ListSessionsError> {
    let sessions = get_sessions(pool).await?;

    println!(
        "{}",
        serde_json::to_string(&sessions).context("couldn't serialize response")?
    );

    Ok(())
}

pub async fn handle_list_crumbs(
    pool: &Pool<Sqlite>,
    session_id: i64,
) -> Result<(), ListCrumbsError> {
    let crumbs = get_crumbs(pool, session_id).await?;

    println!(
        "{}",
        serde_json::to_string(&crumbs).context("couldn't serialize response")?
    );

    Ok(())
}

pub async fn handle_tui(pool: Pool<Sqlite>, theme: ThemeName) -> anyhow::Result<()> {
    crate::tui::run(pool, TuiConfig { theme }).await
}

fn unix_timestamp_now() -> anyhow::Result<i64> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before unix epoch")?;

    i64::try_from(duration.as_secs()).context("unix timestamp overflowed i64")
}
