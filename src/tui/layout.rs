use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub const MIN_TERMINAL_WIDTH: u16 = 80;
pub const MIN_TERMINAL_HEIGHT: u16 = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Sessions,
    Crumbs,
    Metadata,
    Help,
}

impl Pane {
    pub fn next(self) -> Self {
        match self {
            Self::Sessions => Self::Crumbs,
            Self::Crumbs => Self::Metadata,
            Self::Metadata | Self::Help => Self::Sessions,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::Sessions => Self::Metadata,
            Self::Crumbs => Self::Sessions,
            Self::Metadata => Self::Crumbs,
            Self::Help => Self::Sessions,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TerminalDimensions {
    pub width: u16,
    pub height: u16,
}

impl TerminalDimensions {
    pub fn update(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn area(self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MainViewLayout {
    pub footer: Rect,
    pub sessions: Rect,
    pub crumbs: Rect,
    pub metadata: Rect,
}

pub fn main_view_layout(area: Rect) -> MainViewLayout {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(area);
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(root[0]);
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(columns[1]);

    MainViewLayout {
        footer: root[1],
        sessions: columns[0],
        crumbs: right[0],
        metadata: right[1],
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HelpViewLayout {
    pub help: Rect,
    pub footer: Rect,
}

pub fn help_view_layout(area: Rect) -> HelpViewLayout {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(1)])
        .split(area);

    HelpViewLayout {
        help: root[0],
        footer: root[1],
    }
}

pub fn available_crumbs_pane_height(area: Rect) -> usize {
    usize::from(main_view_layout(area).crumbs.height.saturating_sub(3))
}

pub fn available_help_pane_height(area: Rect) -> usize {
    usize::from(help_view_layout(area).help.height.saturating_sub(3))
}
