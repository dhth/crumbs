use crate::args::{Args, CrumbsCommand};
use crate::errors::AppError;
use crate::persistence::get_db_pool;
use anyhow::Context;
use clap::Parser;
use etcetera::{BaseStrategy, choose_base_strategy};
use std::path::{Path, PathBuf};

pub async fn run() -> Result<(), AppError> {
    let args = Args::parse();

    if let CrumbsCommand::WriteSkill = args.command {
        crate::cmds::handle_write_skill()?;
        return Ok(());
    }

    let crumbs_db_url = get_db_url(args.db_path)?;
    let pool = get_db_pool(&crumbs_db_url).await?;

    match args.command {
        CrumbsCommand::Register { agent_name, title } => {
            crate::cmds::handle_register_session(&pool, agent_name, title).await?;
        }
        CrumbsCommand::Log {
            session_id,
            message,
            state,
            confidence,
        } => {
            crate::cmds::handle_log_crumb(&pool, session_id, message, state, confidence).await?;
        }
        CrumbsCommand::Sessions => {
            crate::cmds::handle_list_sessions(&pool).await?;
        }
        CrumbsCommand::List { session_id } => {
            crate::cmds::handle_list_crumbs(&pool, session_id).await?;
        }
        CrumbsCommand::Tui { theme } => {
            crate::cmds::handle_tui(pool, theme).await?;
        }
        CrumbsCommand::WriteSkill => {}
    }

    Ok(())
}

fn get_db_url(db_path: Option<PathBuf>) -> anyhow::Result<String> {
    let db_path = match db_path {
        Some(db_path) => db_path,
        None => get_default_db_path()?,
    };

    ensure_parent_dir_exists(&db_path)?;

    Ok(format!("sqlite://{}", db_path.to_string_lossy()))
}

fn get_default_db_path() -> anyhow::Result<PathBuf> {
    let strategy = choose_base_strategy().context("couldn't determine your data directory")?;
    let data_dir = strategy.data_dir().join("crumbs");

    Ok(data_dir.join("crumbs.db"))
}

fn ensure_parent_dir_exists(path: &Path) -> anyhow::Result<()> {
    let Some(parent_dir) = path.parent() else {
        return Ok(());
    };

    if parent_dir.as_os_str().is_empty() {
        return Ok(());
    }

    std::fs::create_dir_all(parent_dir).with_context(|| {
        format!(
            "couldn't create database directory: {}",
            parent_dir.to_string_lossy()
        )
    })?;

    Ok(())
}
