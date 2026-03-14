#[derive(Debug, Clone)]
pub enum Cmd {
    ArchiveSession { session_id: i64 },
    LoadCrumbsForSession { session_id: i64 },
    LoadSessions,
}
