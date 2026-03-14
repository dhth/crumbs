use super::cmd::Cmd;
use super::model::current_timestamp;
use super::msg::Msg;
use crate::persistence::{archive_session, get_crumbs, get_sessions};
use sqlx::{Pool, Sqlite};
use tokio::sync::mpsc::Sender;

pub async fn handle_command(command: Cmd, pool: Pool<Sqlite>, event_tx: Sender<Msg>) {
    match command {
        Cmd::ArchiveSession { session_id } => {
            tokio::spawn(async move {
                let result = archive_session(&pool, session_id, current_timestamp())
                    .await
                    .map_err(|error| error.to_string());
                let msg = Msg::SessionArchived { session_id, result };

                let _ = event_tx.send(msg).await;
            });
        }
        Cmd::LoadSessions => {
            tokio::spawn(async move {
                let result = get_sessions(&pool).await.map_err(|error| error.to_string());
                let msg = Msg::SessionsLoaded(result);

                let _ = event_tx.send(msg).await;
            });
        }
        Cmd::LoadCrumbsForSession { session_id } => {
            tokio::spawn(async move {
                let result = get_crumbs(&pool, session_id)
                    .await
                    .map_err(|error| error.to_string());
                let msg = Msg::CrumbsLoaded { session_id, result };

                let _ = event_tx.send(msg).await;
            });
        }
    }
}
