use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(i64);

impl SessionId {
    pub fn get(self) -> i64 {
        self.0
    }
}

impl TryFrom<i64> for SessionId {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value <= 0 {
            return Err("session id must be positive");
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentName(String);

impl AgentName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for AgentName {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err("agent name cannot be empty");
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionTitle(String);

impl SessionTitle {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for SessionTitle {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err("session title cannot be empty");
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CrumbMessage(String);

impl CrumbMessage {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for CrumbMessage {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.trim().is_empty() {
            return Err("crumb message cannot be empty");
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Confidence(u8);

impl Confidence {
    pub fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Confidence {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 100 {
            return Err("confidence must be between 0 and 100");
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ValueEnum)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    Working,
    Blocked,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: i64,
    pub agent_name: String,
    pub title: String,
    pub project_name: String,
    pub path: String,
    pub branch: Option<String>,
    pub state: SessionState,
    pub archived_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionToSave {
    pub agent_name: AgentName,
    pub title: SessionTitle,
    pub project_name: String,
    pub path: PathBuf,
    pub branch: Option<String>,
    pub state: SessionState,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Crumb {
    pub id: i64,
    pub session_id: i64,
    pub message: String,
    pub state: Option<SessionState>,
    pub confidence: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrumbToSave {
    pub session_id: SessionId,
    pub message: CrumbMessage,
    pub state: Option<SessionState>,
    pub confidence: Option<Confidence>,
    pub timestamp: i64,
}
