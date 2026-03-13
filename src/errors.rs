use crate::cmds::{ListCrumbsError, ListSessionsError, LogCrumbError, RegisterSessionError};
use crate::persistence::{AddCrumbError, DBPoolError, GetCrumbsError};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    DBPool(#[from] DBPoolError),
    #[error(transparent)]
    RegisterSession(#[from] RegisterSessionError),
    #[error(transparent)]
    LogCrumb(#[from] LogCrumbError),
    #[error(transparent)]
    ListSessions(#[from] ListSessionsError),
    #[error(transparent)]
    ListCrumbs(#[from] ListCrumbsError),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl AppError {
    pub fn follow_up(&self) -> Option<&'static str> {
        match self {
            AppError::DBPool(_) => None,
            AppError::RegisterSession(register_error) => match register_error {
                RegisterSessionError::InvalidInput(_) => None,
                RegisterSessionError::CouldntGetCwd(_) => Some(
                    "Tip: Make sure the current working directory exists and is accessible, then try again.",
                ),
                RegisterSessionError::Persistence(_) => None,
                RegisterSessionError::Unexpected(_) => None,
            },
            AppError::LogCrumb(log_error) => match log_error {
                LogCrumbError::InvalidInput(_) => None,
                LogCrumbError::Persistence(add_crumb_error) => match add_crumb_error {
                    AddCrumbError::SessionDoesNotExist(_) => Some(
                        r#"Tip: Use the session ID returned by 'crumbs register', or run 'crumbs sessions' to view all sessions."#,
                    ),
                    AddCrumbError::Sqlx(_) => None,
                },
                LogCrumbError::Unexpected(_) => None,
            },
            AppError::ListSessions(_) => None,
            AppError::ListCrumbs(list_error) => match list_error {
                ListCrumbsError::Persistence(get_crumbs_error) => match get_crumbs_error {
                    GetCrumbsError::SessionDoesNotExist(_) => {
                        Some(r#"Tip: Use 'crumbs sessions' to view all sessions."#)
                    }
                    GetCrumbsError::Sqlx(_) => None,
                },
                ListCrumbsError::Unexpected(_) => None,
            },
            AppError::Unexpected(_) => None,
        }
    }
}
