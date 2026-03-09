use crate::domain::SessionState;
use clap::{Parser, Subcommand};

/// crumbs lets your AI agents report progress to you
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: CrumbsCommand,
}

#[derive(Subcommand, Debug)]
pub enum CrumbsCommand {
    /// register a new session
    Register {
        /// the name of the agent
        #[arg(value_name = "NAME")]
        agent_name: String,
        /// title for the session
        #[arg(value_name = "TITLE")]
        title: String,
    },
    /// log a crumb in a session
    Log {
        /// id of the session to add the log to
        #[arg(value_name = "ID")]
        session_id: i64,
        /// message to save as a crumb
        #[arg(value_name = "STRING")]
        message: String,
        /// session state to set
        #[arg(short = 's', long = "state")]
        state: Option<SessionState>,
    },
    /// list all sessions
    Sessions,
    /// list all crumbs for a session
    List {
        /// id of the session to list crumbs for
        #[arg(value_name = "ID")]
        session_id: i64,
    },
}
