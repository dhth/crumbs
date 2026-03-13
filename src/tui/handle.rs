use super::cmd::Cmd;
use super::msg::Msg;
use crate::persistence::{get_crumbs, get_sessions};
use sqlx::{Pool, Sqlite};
use tokio::sync::mpsc::Sender;

pub async fn handle_command(command: Cmd, pool: Pool<Sqlite>, event_tx: Sender<Msg>) {
    match command {
        Cmd::LoadSessions => {
            tokio::spawn(async move {
                let msg = Msg::SessionsLoaded(
                    get_sessions(&pool).await.map_err(|error| error.to_string()),
                );

                let _ = event_tx.send(msg).await;
            });
        }
        Cmd::LoadCrumbsForSession { session_id } => {
            tokio::spawn(async move {
                let msg = Msg::CrumbsLoaded {
                    session_id,
                    result: get_crumbs(&pool, session_id)
                        .await
                        .map_err(|error| error.to_string()),
                };

                let _ = event_tx.send(msg).await;
            });
        }
    }
}
