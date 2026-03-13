#[derive(Debug, Clone)]
pub enum Cmd {
    LoadCrumbsForSession { session_id: i64 },
    LoadSessions,
}
