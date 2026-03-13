use super::cmd::Cmd;
use super::handle::handle_command;
use super::layout::TerminalDimensions;
use super::model::{Model, RunningState};
use super::msg::{Msg, get_event_handling_msg};
use super::update::update;
use super::view::view;
use crate::config::ThemeName;
use anyhow::Context;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::poll;
use ratatui::{Terminal, try_restore};
use sqlx::{Pool, Sqlite};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time;

const EVENT_POLL_DURATION_MS: u64 = 16;
const SESSIONS_REFRESH_INTERVAL_SECS: u64 = 8;
const CRUMBS_REFRESH_INTERVAL_SECS: u64 = 5;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TuiConfig {
    pub theme: ThemeName,
}

pub async fn run(pool: Pool<Sqlite>, config: TuiConfig) -> anyhow::Result<()> {
    let mut tui = Tui::new(pool, config)?;
    tui.run().await
}

struct Tui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pool: Pool<Sqlite>,
    event_tx: Sender<Msg>,
    event_rx: Receiver<Msg>,
    model: Model,
}

impl Tui {
    fn new(pool: Pool<Sqlite>, config: TuiConfig) -> anyhow::Result<Self> {
        let terminal = ratatui::try_init()?;
        let (event_tx, event_rx) = mpsc::channel(16);
        let (width, height) = ratatui::crossterm::terminal::size()?;
        let debug = std::env::var("CRUMBS_DEBUG")
            .map(|v| v == "1")
            .unwrap_or(false);
        let model = Model::new(TerminalDimensions { width, height }, config.theme, debug);

        Ok(Self {
            terminal,
            pool,
            event_tx,
            event_rx,
            model,
        })
    }

    async fn run(&mut self) -> anyhow::Result<()> {
        let result = self.run_inner().await;

        if let Err(restore_error) = try_restore()
            && result.is_ok()
        {
            return Err(restore_error).context("couldn't restore terminal to its original state");
        }

        result
    }

    async fn run_inner(&mut self) -> anyhow::Result<()> {
        let _ = self.terminal.clear();
        self.model.render_counter += 1;
        self.terminal.draw(|frame| view(&mut self.model, frame))?;

        let initial_cmds = vec![Cmd::LoadSessions];
        for cmd in initial_cmds {
            handle_command(cmd, self.pool.clone(), self.event_tx.clone()).await;
        }

        let mut sessions_refresh_interval =
            time::interval(Duration::from_secs(SESSIONS_REFRESH_INTERVAL_SECS));
        let mut crumbs_refresh_interval =
            time::interval(Duration::from_secs(CRUMBS_REFRESH_INTERVAL_SECS));

        loop {
            tokio::select! {
                Some(message) = self.event_rx.recv() => {
                    let cmds = update(&mut self.model, message);

                    if self.model.running_state == RunningState::Done {
                        break;
                    }

                    self.terminal.draw(|frame| view(&mut self.model, frame))?;
                    self.model.render_counter += 1;

                    for cmd in cmds {
                        handle_command(cmd, self.pool.clone(), self.event_tx.clone()).await;
                    }
                }

                _ = sessions_refresh_interval.tick() => {
                    self.event_tx.send(Msg::RefreshSessions).await?;
                }

                _ = crumbs_refresh_interval.tick() => {
                    if let Some(session_id) = self.model.sessions.selected_session_id() {
                        self.event_tx.send(Msg::RefreshCrumbs(session_id)).await?;
                    }
                }

                Ok(poll_result) = tokio::task::spawn_blocking(|| poll(Duration::from_millis(EVENT_POLL_DURATION_MS))) => {
                    match poll_result {
                        Ok(true) => {
                            // non blocking read since poll returned Ok(true)
                            let event = ratatui::crossterm::event::read()?;
                            self.model.event_counter += 1;
                            if let Some(msg) = get_event_handling_msg(&self.model, event) {
                                self.event_tx.try_send(msg)?;
                            }
                        }
                        Ok(false) => continue,
                        Err(error) => return Err(anyhow::anyhow!(error).context("couldn't poll for events")),
                    }
                }
            }
        }

        Ok(())
    }
}
