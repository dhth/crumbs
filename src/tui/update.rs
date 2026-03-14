use super::cmd::Cmd;
use super::layout::{MIN_TERMINAL_HEIGHT, MIN_TERMINAL_WIDTH, Pane};
use super::model::{MetadataPaneContents, Model, RunningState, UserMsg, current_timestamp};
use super::msg::Msg;

const CRUMB_PREFETCH_RADIUS: usize = 2;

pub fn update(model: &mut Model, msg: Msg) -> Vec<Cmd> {
    model.current_timestamp = current_timestamp();
    let previous_selected_session_id = model.sessions.selected_session_id();
    let mut cmds = Vec::new();

    match msg {
        Msg::ArchiveSelectedSession => {
            if let Some(session_id) = model.sessions.selected_session_id() {
                cmds.push(Cmd::ArchiveSession { session_id });
            }
        }
        Msg::CrumbsLoaded { session_id, result } => {
            model.clear_crumb_load_in_flight(session_id);

            if model.has_session(session_id) {
                match result {
                    Ok(crumbs) => {
                        model.cache_crumbs(session_id, crumbs);

                        if model.sessions.selected_session_id() == Some(session_id) {
                            model.recompute_crumbs_scroll_bounds();
                        }
                    }
                    Err(error) => {
                        model.user_msg =
                            Some(UserMsg::error(format!("loading crumbs failed: {error}")));
                    }
                }
            }
        }
        Msg::FocusNextPane => model.active_pane = model.active_pane.next(),
        Msg::FocusPreviousPane => model.active_pane = model.active_pane.previous(),
        Msg::GoBackOrQuit => match model.active_pane {
            Pane::Help => model.close_help(),
            Pane::Metadata => model.active_pane = Pane::Crumbs,
            Pane::Crumbs => model.active_pane = Pane::Sessions,
            Pane::Sessions => model.running_state = RunningState::Done,
        },
        Msg::OpenHelp => {
            if model.active_pane == super::layout::Pane::Help {
                model.close_help();
            } else {
                model.open_help();
            }
        }
        Msg::RefreshCrumbs(session_id) => {
            if let Some(cmd) = crumb_load_cmd(model, session_id, true) {
                cmds.push(cmd);
            }
        }
        Msg::RefreshSessions => cmds.push(Cmd::LoadSessions),
        Msg::QuitImmediately => model.running_state = RunningState::Done,
        Msg::ScrollCrumbsDown => model.scroll_crumbs_down(),
        Msg::ScrollCrumbsUp => model.scroll_crumbs_up(),
        Msg::ScrollHelpDown => model.scroll_help_down(),
        Msg::ScrollHelpUp => model.scroll_help_up(),
        Msg::SessionArchived {
            session_id: archived_session_id,
            result,
        } => match result {
            Ok(_) => {
                model.remove_session(archived_session_id);
                model.remove_cached_crumbs(archived_session_id);
                model.clear_crumb_load_in_flight(archived_session_id);

                model.user_msg = Some(UserMsg::info(format!(
                    "archived session {archived_session_id}"
                )));
                cmds.push(Cmd::LoadSessions);
            }
            Err(error) => {
                model.user_msg = Some(UserMsg::error(format!("archiving session failed: {error}")));
            }
        },
        Msg::SelectNextSession => model.sessions.select_next(),
        Msg::SelectNextTheme => {
            model.theme = model.theme.next();
            model.user_msg = Some(UserMsg::info(format!(
                "theme changed to {}",
                model.theme.as_str()
            )));
        }
        Msg::SelectPreviousSession => model.sessions.select_previous(),
        Msg::SelectPreviousTheme => {
            model.theme = model.theme.previous();
            model.user_msg = Some(UserMsg::info(format!(
                "theme changed to {}",
                model.theme.as_str()
            )));
        }
        Msg::SessionsLoaded(result) => match result {
            Ok(sessions) => {
                model.set_sessions(sessions);
            }
            Err(error) => {
                model.user_msg = Some(UserMsg::error(format!("loading sessions failed: {error}")));
            }
        },
        Msg::TerminalResize(width, height) => {
            let height_changed = model.terminal_dimensions.height != height;
            let was_too_small = model.terminal_too_small;
            model.terminal_dimensions.update(width, height);
            model.terminal_too_small = width < MIN_TERMINAL_WIDTH || height < MIN_TERMINAL_HEIGHT;

            if height_changed || was_too_small != model.terminal_too_small {
                model.recompute_crumbs_scroll_bounds();
                model.recompute_help_scroll_bounds();
            }
        }
        Msg::ToggleTimeDisplayMode => model.toggle_time_display_mode(),
    }

    match (
        model.sessions.selected_session(),
        previous_selected_session_id,
    ) {
        (None, Some(_)) => {
            model.clear_metadata_pane_contents();
            model.refresh_crumbs_scroll();
        }
        (Some(selected_session), Some(previous)) if selected_session.id != previous => {
            model.set_metadata_pane_contents(MetadataPaneContents::from(selected_session));
            model.refresh_crumbs_scroll();
        }
        (Some(selected_session), None) => {
            model.set_metadata_pane_contents(MetadataPaneContents::from(selected_session));
            model.refresh_crumbs_scroll();
        }
        (Some(selected_session), _) => {
            model.set_metadata_pane_contents(MetadataPaneContents::from(selected_session));
        }
        (None, None) => {}
    }

    cmds.extend(prefetch_crumbs_commands(model));

    model.clear_selected_session_marker();

    if let Some(message) = &mut model.user_msg {
        let clear = if message.frames_left == 0 {
            true
        } else {
            message.frames_left -= 1;
            false
        };

        if clear {
            model.user_msg = None;
        }
    }

    cmds
}

fn prefetch_crumbs_commands(model: &mut Model) -> Vec<Cmd> {
    let Some(selected_index) = model.sessions.selected_index() else {
        return Vec::new();
    };

    let start = selected_index.saturating_sub(CRUMB_PREFETCH_RADIUS);
    let end = selected_index
        .saturating_add(CRUMB_PREFETCH_RADIUS)
        .min(model.sessions.len().saturating_sub(1));
    let mut cmds = Vec::new();

    for index in start..=end {
        if let Some(session_id) = model.sessions.session_id_at(index)
            && let Some(cmd) = crumb_load_cmd(model, session_id, false)
        {
            cmds.push(cmd);
        }
    }

    cmds
}

fn crumb_load_cmd(model: &mut Model, session_id: i64, force_reload: bool) -> Option<Cmd> {
    if model.has_crumb_load_in_flight(session_id) {
        return None;
    }

    if !force_reload && model.has_cached_crumbs(session_id) {
        return None;
    }

    model.mark_crumb_load_in_flight(session_id);
    Some(Cmd::LoadCrumbsForSession { session_id })
}
