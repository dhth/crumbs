use super::layout::Pane;
use super::model::Model;
use crate::domain::{Crumb, Session};
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

pub enum Msg {
    // user actions
    FocusNextPane,
    FocusPreviousPane,
    GoBackOrQuit,
    OpenHelp,
    QuitImmediately,
    ScrollCrumbsDown,
    ScrollCrumbsUp,
    ScrollHelpDown,
    ScrollHelpUp,
    SelectNextSession,
    SelectNextTheme,
    SelectPreviousSession,
    SelectPreviousTheme,
    ToggleTimeDisplayMode,
    // internal
    CrumbsLoaded {
        session_id: i64,
        result: Result<Vec<Crumb>, String>,
    },
    RefreshCrumbs(i64),
    RefreshSessions,
    SessionsLoaded(Result<Vec<Session>, String>),
    TerminalResize(u16, u16),
}

pub fn get_event_handling_msg(model: &Model, event: Event) -> Option<Msg> {
    match event {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            if key_event.code == KeyCode::Char('c') && key_event.modifiers == KeyModifiers::CONTROL
            {
                return Some(Msg::QuitImmediately);
            }

            if model.terminal_too_small {
                return match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => Some(Msg::QuitImmediately),
                    _ => None,
                };
            }

            match model.active_pane {
                Pane::Help => match key_event.code {
                    KeyCode::Char('j') | KeyCode::Down => Some(Msg::ScrollHelpDown),
                    KeyCode::Char('k') | KeyCode::Up => Some(Msg::ScrollHelpUp),
                    KeyCode::Char('{') => Some(Msg::SelectPreviousTheme),
                    KeyCode::Char('}') => Some(Msg::SelectNextTheme),
                    KeyCode::Char('?') => Some(Msg::OpenHelp),
                    KeyCode::Esc | KeyCode::Char('q') => Some(Msg::GoBackOrQuit),
                    _ => None,
                },
                _ => match key_event.code {
                    KeyCode::Char('j') | KeyCode::Down => match model.active_pane {
                        Pane::Sessions => Some(Msg::SelectNextSession),
                        Pane::Crumbs => Some(Msg::ScrollCrumbsDown),
                        Pane::Metadata | Pane::Help => None,
                    },
                    KeyCode::Char('k') | KeyCode::Up => match model.active_pane {
                        Pane::Sessions => Some(Msg::SelectPreviousSession),
                        Pane::Crumbs => Some(Msg::ScrollCrumbsUp),
                        Pane::Metadata | Pane::Help => None,
                    },
                    KeyCode::Char('[') => Some(Msg::SelectPreviousSession),
                    KeyCode::Char(']') => Some(Msg::SelectNextSession),
                    KeyCode::Char('{') => Some(Msg::SelectPreviousTheme),
                    KeyCode::Char('}') => Some(Msg::SelectNextTheme),
                    KeyCode::Char('J') => match model.active_pane {
                        Pane::Sessions => Some(Msg::ScrollCrumbsDown),
                        Pane::Crumbs | Pane::Metadata | Pane::Help => None,
                    },
                    KeyCode::Char('K') => match model.active_pane {
                        Pane::Sessions => Some(Msg::ScrollCrumbsUp),
                        Pane::Crumbs | Pane::Metadata | Pane::Help => None,
                    },
                    KeyCode::Char('?') => Some(Msg::OpenHelp),
                    KeyCode::Char('t') => Some(Msg::ToggleTimeDisplayMode),
                    KeyCode::Tab => Some(Msg::FocusNextPane),
                    KeyCode::BackTab => Some(Msg::FocusPreviousPane),
                    KeyCode::Esc | KeyCode::Char('q') => Some(Msg::GoBackOrQuit),
                    _ => None,
                },
            }
        }
        Event::Resize(width, height) => Some(Msg::TerminalResize(width, height)),
        _ => None,
    }
}
