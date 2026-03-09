use crate::args::{Args, CrumbsCommand};
use crate::errors::AppError;
use crate::persistence::get_db_pool;
use anyhow::Context;
use clap::Parser;
use etcetera::{BaseStrategy, choose_base_strategy};

pub async fn run() -> Result<(), AppError> {
    let args = Args::parse();

    let crumbs_db_url = get_default_db_url()?;
    let pool = get_db_pool(&crumbs_db_url).await?;

    match args.command {
        CrumbsCommand::Register { agent_name, title } => {
            crate::cmds::handle_register_session(&pool, agent_name, title).await?;
        }
        CrumbsCommand::Log {
            session_id,
            message,
            state,
        } => {
            crate::cmds::handle_log_crumb(&pool, session_id, message, state).await?;
        }
        CrumbsCommand::Sessions => {
            crate::cmds::handle_list_sessions(&pool).await?;
        }
        CrumbsCommand::List { session_id } => {
            crate::cmds::handle_list_crumbs(&pool, session_id).await?;
        }
    }

    Ok(())
}

fn get_default_db_url() -> anyhow::Result<String> {
    let strategy = choose_base_strategy().context("couldn't determine your data directory")?;
    let data_dir = strategy.data_dir().join("crumbs");

    std::fs::create_dir_all(&data_dir).with_context(|| {
        format!(
            "couldn't create crumbs' data directory: {}",
            data_dir.to_string_lossy()
        )
    })?;

    Ok(format!(
        "sqlite://{}",
        data_dir.join("crumbs.db").to_string_lossy()
    ))
}
