use super::help::help_line_count;
use super::layout::{
    MIN_TERMINAL_HEIGHT, MIN_TERMINAL_WIDTH, Pane, TerminalDimensions,
    available_crumbs_pane_height, available_help_pane_height,
};
use crate::config::ThemeName;
use crate::domain::{Crumb, Session, SessionState};
use ratatui::widgets::ListState;
use std::collections::{HashMap, HashSet};

const USER_MESSAGE_DEFAULT_FRAMES: u16 = 4;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TimeDisplayMode {
    #[default]
    Relative,
    Absolute,
}

impl TimeDisplayMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Relative => Self::Absolute,
            Self::Absolute => Self::Relative,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    Info,
    Error,
}

#[derive(Debug, Clone)]
pub struct UserMsg {
    pub frames_left: u16,
    pub value: String,
    pub kind: MessageKind,
}

impl UserMsg {
    pub fn info<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            frames_left: USER_MESSAGE_DEFAULT_FRAMES,
            value: message.into(),
            kind: MessageKind::Info,
        }
    }

    pub fn error<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            frames_left: USER_MESSAGE_DEFAULT_FRAMES,
            value: message.into(),
            kind: MessageKind::Error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetadataPaneContents {
    pub title: String,
    pub agent_name: String,
    pub project_name: String,
    pub path: String,
    pub branch: Option<String>,
    pub state: String,
    pub started_at: i64,
    pub last_crumb_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionMarker {
    New,
    Updated,
}

#[derive(Debug, Clone)]
pub struct SessionListItem {
    pub session: Session,
    pub marker: Option<SessionMarker>,
}

#[derive(Debug, Default)]
pub struct Sessions {
    items: Vec<SessionListItem>,
    state: ListState,
}

impl Sessions {
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn items(&self) -> &[SessionListItem] {
        &self.items
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn selected_session(&self) -> Option<&Session> {
        self.selected_index()
            .and_then(|index| self.items.get(index))
            .map(|item| &item.session)
    }

    pub fn selected_session_id(&self) -> Option<i64> {
        self.selected_session().map(|session| session.id)
    }

    pub fn session_id_at(&self, index: usize) -> Option<i64> {
        self.items.get(index).map(|item| item.session.id)
    }

    pub fn set_items(&mut self, sessions: Vec<Session>) {
        let previous_selected_session_id = self.selected_session_id();
        let was_previously_empty = self.is_empty();
        let previous_sessions: HashMap<i64, (i64, Option<SessionMarker>)> = self
            .items
            .iter()
            .map(|item| (item.session.id, (item.session.updated_at, item.marker)))
            .collect();

        self.items = sessions
            .into_iter()
            .map(|session| {
                let marker =
                    if previous_selected_session_id == Some(session.id) || was_previously_empty {
                        None
                    } else if let Some((previous_updated_at, previous_marker)) =
                        previous_sessions.get(&session.id)
                    {
                        if session.updated_at > *previous_updated_at {
                            Some(SessionMarker::Updated)
                        } else {
                            *previous_marker
                        }
                    } else {
                        Some(SessionMarker::New)
                    };

                SessionListItem { session, marker }
            })
            .collect();

        let selected = previous_selected_session_id
            .and_then(|session_id| self.index_of_session(session_id))
            .or_else(|| (!self.is_empty()).then_some(0));

        self.state = self.state.with_selected(selected);
        self.clear_selected_marker();
    }

    pub fn select_next(&mut self) {
        if self.is_empty() {
            return;
        }

        match self.selected_index() {
            Some(index) if index + 1 < self.len() => {
                self.state.select(Some(index + 1));
            }
            Some(_) => {}
            None => {
                self.state.select(Some(0));
            }
        }
    }

    pub fn select_previous(&mut self) {
        if self.is_empty() {
            return;
        }

        match self.selected_index() {
            Some(index) if index > 0 => {
                self.state.select(Some(index - 1));
            }
            Some(_) => {}
            None => {
                self.state.select(Some(0));
            }
        }
    }

    pub fn clear_selected_marker(&mut self) {
        if let Some(item) = self
            .selected_index()
            .and_then(|index| self.items.get_mut(index))
        {
            item.marker = None;
        }
    }

    fn index_of_session(&self, session_id: i64) -> Option<usize> {
        self.items
            .iter()
            .position(|item| item.session.id == session_id)
    }
}

impl From<&Session> for MetadataPaneContents {
    fn from(session: &Session) -> Self {
        Self {
            title: session.title.clone(),
            agent_name: session.agent_name.clone(),
            project_name: session.project_name.clone(),
            path: session.path.clone(),
            branch: session.branch.clone(),
            state: session_state_label(&session.state).to_string(),
            started_at: session.created_at,
            last_crumb_at: session.updated_at,
        }
    }
}

#[derive(Debug)]
pub struct Model {
    pub active_pane: Pane,
    pub last_active_pane: Option<Pane>,
    pub current_timestamp: i64,
    pub running_state: RunningState,
    pub theme: ThemeName,
    pub time_display_mode: TimeDisplayMode,
    pub sessions: Sessions,
    pub crumb_cache: HashMap<i64, Vec<Crumb>>,
    crumb_loads_in_flight: HashSet<i64>,
    crumbs_scroll: usize,
    max_crumbs_scroll_available: usize,
    help_scroll: usize,
    help_line_count: usize,
    max_help_scroll_available: usize,
    pub metadata_pane_contents: Option<MetadataPaneContents>,
    pub status_message: Option<String>,
    pub user_msg: Option<UserMsg>,
    pub terminal_dimensions: TerminalDimensions,
    pub terminal_too_small: bool,
    pub event_counter: u64,
    pub render_counter: u64,
    pub debug: bool,
}

impl Model {
    pub fn new(terminal_dimensions: TerminalDimensions, theme: ThemeName, debug: bool) -> Self {
        let terminal_too_small = terminal_dimensions.width < MIN_TERMINAL_WIDTH
            || terminal_dimensions.height < MIN_TERMINAL_HEIGHT;

        let mut model = Self {
            active_pane: Pane::Sessions,
            last_active_pane: None,
            current_timestamp: current_timestamp(),
            running_state: RunningState::Running,
            theme,
            time_display_mode: TimeDisplayMode::Relative,
            sessions: Sessions::default(),
            crumb_cache: HashMap::new(),
            crumb_loads_in_flight: HashSet::new(),
            crumbs_scroll: 0,
            max_crumbs_scroll_available: 0,
            help_scroll: 0,
            help_line_count: help_line_count(),
            max_help_scroll_available: 0,
            metadata_pane_contents: None,
            status_message: None,
            user_msg: None,
            terminal_dimensions,
            terminal_too_small,
            event_counter: 0,
            render_counter: 0,
            debug,
        };

        model.recompute_help_scroll_bounds();

        model
    }

    pub fn open_help(&mut self) {
        if self.active_pane == Pane::Help {
            return;
        }

        self.last_active_pane = Some(self.active_pane);
        self.active_pane = Pane::Help;
        self.help_scroll = 0;
    }

    pub fn close_help(&mut self) {
        if self.active_pane != Pane::Help {
            return;
        }

        self.active_pane = self.last_active_pane.unwrap_or(Pane::Sessions);
        self.last_active_pane = None;
    }

    pub fn set_sessions(&mut self, sessions: Vec<Session>) {
        self.sessions.set_items(sessions);
    }

    pub fn toggle_time_display_mode(&mut self) {
        self.time_display_mode = self.time_display_mode.toggle();
    }

    pub fn cache_crumbs(&mut self, session_id: i64, crumbs: Vec<Crumb>) {
        self.crumb_cache.insert(session_id, crumbs);
    }

    pub fn mark_crumb_load_in_flight(&mut self, session_id: i64) {
        self.crumb_loads_in_flight.insert(session_id);
    }

    pub fn clear_crumb_load_in_flight(&mut self, session_id: i64) {
        self.crumb_loads_in_flight.remove(&session_id);
    }

    pub fn set_metadata_pane_contents(&mut self, contents: MetadataPaneContents) {
        self.metadata_pane_contents = Some(contents);
    }

    pub fn clear_metadata_pane_contents(&mut self) {
        self.metadata_pane_contents = None;
    }

    pub fn has_cached_crumbs(&self, session_id: i64) -> bool {
        self.crumb_cache.contains_key(&session_id)
    }

    pub fn has_crumb_load_in_flight(&self, session_id: i64) -> bool {
        self.crumb_loads_in_flight.contains(&session_id)
    }

    pub fn current_crumbs(&self) -> Option<&[Crumb]> {
        self.sessions
            .selected_session_id()
            .and_then(|session_id| self.crumb_cache.get(&session_id))
            .map(Vec::as_slice)
    }

    pub fn has_current_crumbs(&self) -> bool {
        self.current_crumbs()
            .is_some_and(|crumbs| !crumbs.is_empty())
    }

    pub fn crumbs_scroll(&self) -> usize {
        self.crumbs_scroll
    }

    pub fn help_scroll(&self) -> usize {
        self.help_scroll
    }

    pub fn clear_selected_session_marker(&mut self) {
        self.sessions.clear_selected_marker();
    }

    pub fn scroll_crumbs_down(&mut self) {
        self.crumbs_scroll = (self.crumbs_scroll + 1).min(self.max_crumbs_scroll_available);
    }

    pub fn scroll_crumbs_up(&mut self) {
        self.crumbs_scroll = self.crumbs_scroll.saturating_sub(1);
    }

    pub fn refresh_crumbs_scroll(&mut self) {
        self.crumbs_scroll = 0;
        self.recompute_crumbs_scroll_bounds();
    }

    pub fn recompute_crumbs_scroll_bounds(&mut self) {
        self.max_crumbs_scroll_available = if self.terminal_too_small {
            0
        } else {
            self.current_crumbs()
                .map_or(0, <[Crumb]>::len)
                .saturating_sub(available_crumbs_pane_height(
                    self.terminal_dimensions.area(),
                ))
        };

        self.crumbs_scroll = self.crumbs_scroll.min(self.max_crumbs_scroll_available);
    }

    pub fn scroll_help_down(&mut self) {
        self.help_scroll = (self.help_scroll + 1).min(self.max_help_scroll_available);
    }

    pub fn scroll_help_up(&mut self) {
        self.help_scroll = self.help_scroll.saturating_sub(1);
    }

    pub fn recompute_help_scroll_bounds(&mut self) {
        self.max_help_scroll_available = if self.terminal_too_small {
            0
        } else {
            self.help_line_count
                .saturating_sub(available_help_pane_height(self.terminal_dimensions.area()))
        };

        self.help_scroll = self.help_scroll.min(self.max_help_scroll_available);
    }
}

pub fn current_timestamp() -> i64 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => i64::try_from(duration.as_secs()).unwrap_or(i64::MAX),
        Err(_) => 0,
    }
}

fn session_state_label(state: &SessionState) -> &'static str {
    match state {
        SessionState::Working => "working",
        SessionState::Blocked => "blocked",
        SessionState::Done => "done",
    }
}
