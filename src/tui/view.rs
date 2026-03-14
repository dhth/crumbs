use super::help::HELP_CONTENT;
use super::layout::{
    MIN_TERMINAL_HEIGHT, MIN_TERMINAL_WIDTH, Pane, TerminalDimensions, help_view_layout,
    main_view_layout,
};
use super::model::{
    MessageKind, MetadataPaneContents, Model, SessionListItem, SessionMarker, TimeDisplayMode,
};
use super::theme::{Theme, get_theme};
use crate::domain::{Crumb, SessionState};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Cell, List, ListDirection, ListItem, Padding, Paragraph, Row, Table, Wrap,
};
use time::OffsetDateTime;
use time::UtcOffset;
use time::macros::format_description;

pub fn view(model: &mut Model, frame: &mut Frame) {
    let theme = get_theme(model.theme);

    if model.terminal_too_small {
        render_terminal_too_small(frame, &model.terminal_dimensions, &theme);
        return;
    }

    if model.active_pane == Pane::Help {
        render_help_view(model, frame, &theme);
    } else {
        render_main_view(model, frame, &theme);
    }
}

fn render_main_view(model: &mut Model, frame: &mut Frame, theme: &Theme) {
    let layout = main_view_layout(frame.area());

    render_sessions_pane(model, frame, layout.sessions, theme);
    render_crumbs_pane(model, frame, layout.crumbs, theme);
    render_metadata_pane(model, frame, layout.metadata, theme);
    render_footer(model, frame, layout.footer, theme);
}

fn render_help_view(model: &Model, frame: &mut Frame, theme: &Theme) {
    let layout = help_view_layout(frame.area());

    render_help_pane(model, frame, layout.help, theme);
    render_footer(model, frame, layout.footer, theme);
}

fn render_sessions_pane(model: &mut Model, frame: &mut Frame, area: Rect, theme: &Theme) {
    let pane_name = " sessions ";
    let style = pane_style(model.active_pane == Pane::Sessions, theme);
    let title = sessions_title(model, pane_name, style.title_color, theme);

    if model.sessions.is_empty() {
        let widget = Paragraph::new("sessions will appear here")
            .block(
                Block::bordered()
                    .title(title)
                    .border_style(Style::default().fg(style.border_color))
                    .padding(Padding::new(1, 0, 1, 0)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });

        frame.render_widget(widget, area);
        return;
    }

    let now = model.current_timestamp;
    let items: Vec<ListItem> = model
        .sessions
        .items()
        .iter()
        .map(|item| session_list_item(item, now, model.time_display_mode, theme))
        .collect();
    let list = List::new(items)
        .block(
            Block::bordered()
                .title(title)
                .border_style(Style::default().fg(style.border_color))
                .padding(Padding::new(0, 0, 1, 0)),
        )
        .highlight_symbol("> ")
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, area, model.sessions.state_mut());
}

fn render_crumbs_pane(model: &Model, frame: &mut Frame, area: Rect, theme: &Theme) {
    let style = pane_style(model.active_pane == Pane::Crumbs, theme);
    let title = crumbs_title(model, style.title_color, theme);

    if model.sessions.selected_session_id().is_none() {
        let widget = Paragraph::new("crumbs for the selected session will appear here")
            .block(
                Block::bordered()
                    .title(title)
                    .border_style(Style::default().fg(style.border_color))
                    .padding(Padding::new(1, 0, 1, 0)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });

        frame.render_widget(widget, area);
    } else if !model.has_current_crumbs() {
        let widget = Paragraph::new("crumbs will appear here")
            .block(
                Block::bordered()
                    .title(title)
                    .border_style(Style::default().fg(style.border_color))
                    .padding(Padding::new(1, 0, 1, 0)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });

        frame.render_widget(widget, area);
    } else {
        let visible_height = usize::from(area.height.saturating_sub(3));

        let rows: Vec<Row> = model
            .current_crumbs()
            .into_iter()
            .flatten()
            .skip(model.crumbs_scroll())
            .take(visible_height)
            .map(|crumb| crumb_row(crumb, theme))
            .collect();

        let widget = Table::new(
            rows,
            [
                Constraint::Length(9),
                Constraint::Length(4),
                Constraint::Fill(1),
            ],
        )
        .block(
            Block::bordered()
                .title(title)
                .border_style(Style::default().fg(style.border_color))
                .padding(Padding::new(1, 0, 1, 0)),
        )
        .column_spacing(2);

        frame.render_widget(widget, area);
    }
}

fn render_metadata_pane(model: &Model, frame: &mut Frame, area: Rect, theme: &Theme) {
    let style = pane_style(model.active_pane == Pane::Metadata, theme);
    match &model.metadata_pane_contents {
        Some(contents) => {
            let widget = Table::new(
                metadata_rows(
                    contents,
                    model.current_timestamp,
                    model.time_display_mode,
                    theme,
                ),
                [Constraint::Length(10), Constraint::Fill(1)],
            )
            .block(
                Block::bordered()
                    .title(
                        " metadata "
                            .bold()
                            .bg(style.title_color)
                            .fg(theme.background),
                    )
                    .border_style(Style::default().fg(style.border_color))
                    .padding(Padding::new(1, 0, 1, 0)),
            )
            .column_spacing(1);

            frame.render_widget(widget, area);
        }
        None => {
            let widget = empty_pane(
                " metadata ",
                "session metadata will appear here",
                style,
                theme,
                true,
            );

            frame.render_widget(widget, area);
        }
    }
}

fn render_help_pane(model: &Model, frame: &mut Frame, area: Rect, theme: &Theme) {
    let lines: Vec<Line> = HELP_CONTENT
        .lines()
        .skip(model.help_scroll())
        .map(Line::raw)
        .collect();

    let widget = Paragraph::new(lines)
        .block(
            Block::bordered()
                .title(" help ".bold().bg(theme.neutral_1).fg(theme.background))
                .border_style(Style::default().fg(theme.neutral_1))
                .padding(Padding::new(1, 0, 1, 0)),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    frame.render_widget(widget, area);
}

fn render_footer(model: &Model, frame: &mut Frame, area: Rect, theme: &Theme) {
    let widget = Paragraph::new(footer_line(model, theme))
        .style(Style::default().fg(theme.muted))
        .alignment(Alignment::Left)
        .block(Block::default());

    frame.render_widget(widget, area);
}

fn empty_pane<'a>(
    title: &'a str,
    body: &'a str,
    style: PaneStyle,
    theme: &Theme,
    pad_top: bool,
) -> Paragraph<'a> {
    let top_padding = if pad_top { 1 } else { 0 };

    Paragraph::new(body)
        .block(
            Block::bordered()
                .title(title.bold().bg(style.title_color).fg(theme.background))
                .border_style(Style::default().fg(style.border_color))
                .padding(Padding::new(1, 0, top_padding, 0)),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
}

struct PaneStyle {
    border_color: Color,
    title_color: Color,
}

fn pane_style(active: bool, theme: &Theme) -> PaneStyle {
    if active {
        PaneStyle {
            border_color: theme.accent,
            title_color: theme.accent,
        }
    } else {
        PaneStyle {
            border_color: theme.neutral_2,
            title_color: theme.neutral_1,
        }
    }
}

fn footer_line(model: &Model, theme: &Theme) -> Line<'static> {
    let mut spans = vec![Span::styled(
        " crumbs ",
        Style::default()
            .bg(theme.accent)
            .fg(theme.background)
            .bold(),
    )];

    if let Some(status_message) = &model.status_message {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            status_message.clone(),
            Style::default().fg(theme.foreground),
        ));
    }

    if let Some(user_msg) = &model.user_msg {
        spans.push(Span::raw(" "));

        let color = match user_msg.kind {
            MessageKind::Info => theme.info,
            MessageKind::Error => theme.error,
        };

        spans.push(Span::styled(
            user_msg.value.clone(),
            Style::default().fg(color),
        ));
    }

    if model.debug {
        spans.push(Span::from(format!(" [event: {}]", model.event_counter)));
        spans.push(Span::from(format!(" [render: {}]", model.render_counter)));
        spans.push(Span::from(format!(
            " [dimensions: {}x{}] ",
            model.terminal_dimensions.width, model.terminal_dimensions.height
        )));
        spans.push(Span::from(format!(
            " [crumbs scroll: {}]",
            model.crumbs_scroll()
        )));
        spans.push(Span::from(format!(
            " [cached: {}]",
            model
                .crumb_cache
                .keys()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )));
    }

    Line::from(spans)
}

fn sessions_title(
    model: &Model,
    pane_name: &str,
    title_color: Color,
    theme: &Theme,
) -> Line<'static> {
    let section_title = Span::from(pane_name.to_string())
        .bold()
        .bg(title_color)
        .fg(theme.background);

    if let Some(index) = model.sessions.selected_index() {
        Line::from(vec![
            section_title,
            Span::from(format!(" ({}/{}) ", index + 1, model.sessions.len())).fg(title_color),
        ])
    } else {
        Line::from(section_title)
    }
}

fn crumbs_title(model: &Model, title_color: Color, theme: &Theme) -> Line<'static> {
    let section_title = Span::from(" crumbs ")
        .bold()
        .bg(title_color)
        .fg(theme.background);

    if let Some(session) = model.sessions.selected_session() {
        Line::from(vec![
            section_title,
            Span::from(format!(" {} ", session.title)).fg(title_color),
        ])
    } else {
        Line::from(section_title)
    }
}

fn crumb_row(crumb: &Crumb, theme: &Theme) -> Row<'static> {
    let state_cell = Cell::from(match crumb.state {
        Some(state) => Line::from(Span::styled(
            format!("[{}]", session_state_label(state)),
            Style::default()
                .bg(session_state_color(state, theme))
                .fg(theme.background)
                .bold(),
        )),
        None => Line::from(""),
    });

    let confidence_cell = Cell::from(match crumb.confidence {
        Some(confidence) => Line::from(Span::styled(
            format!("C{confidence}"),
            Style::default()
                .fg(confidence_color(confidence, theme))
                .bold(),
        )),
        None => Line::from(""),
    });

    let message_cell = Cell::from(crumb.message.clone());

    Row::new([state_cell, confidence_cell, message_cell])
}

fn metadata_rows(
    contents: &MetadataPaneContents,
    now: i64,
    mode: TimeDisplayMode,
    theme: &Theme,
) -> Vec<Row<'static>> {
    let branch = contents.branch.as_deref().unwrap_or("-");

    vec![
        metadata_row("task", &contents.title, theme),
        metadata_row("agent", &contents.agent_name, theme),
        metadata_row("project", &contents.project_name, theme),
        metadata_row("path", &contents.path, theme),
        metadata_row("branch", branch, theme),
        metadata_row("state", &contents.state, theme),
        metadata_row(
            "started",
            &format_time(now, contents.started_at, mode),
            theme,
        ),
        metadata_row(
            "last crumb",
            &format_time(now, contents.last_crumb_at, mode),
            theme,
        ),
    ]
}

fn metadata_row(label: &str, value: &str, theme: &Theme) -> Row<'static> {
    Row::new([
        Cell::from(Line::from(Span::styled(
            label.to_string(),
            Style::default().fg(theme.muted),
        ))),
        Cell::from(value.to_string()),
    ])
}

fn session_list_item(
    item: &SessionListItem,
    now: i64,
    mode: TimeDisplayMode,
    theme: &Theme,
) -> ListItem<'static> {
    let session = &item.session;
    let mut state_spans = vec![Span::styled(
        format!("[{}]", session_state_label(session.state)),
        Style::default()
            .bg(session_state_color(session.state, theme))
            .fg(theme.background)
            .bold(),
    )];

    if let Some(marker) = item.marker {
        let span = match marker {
            SessionMarker::New => Span::styled(
                "[new]",
                Style::default()
                    .bg(theme.accent_alt)
                    .fg(theme.background)
                    .bold(),
            ),
            SessionMarker::Updated => {
                Span::styled("[updated]", Style::default().fg(theme.neutral_1).bold())
            }
        };

        state_spans.push(Span::raw(" "));
        state_spans.push(span);
    }

    let state_line = Line::from(state_spans);

    let mut lines = vec![
        Line::from(session.title.clone()),
        Line::from(session.project_name.clone()),
        state_line,
    ];

    lines.push(Line::from(Span::styled(
        format_time(now, session.updated_at, mode),
        Style::default().fg(theme.muted),
    )));

    ListItem::new(lines)
}

fn format_time(now: i64, timestamp: i64, mode: TimeDisplayMode) -> String {
    match mode {
        TimeDisplayMode::Relative => relative_time_string(now, timestamp),
        TimeDisplayMode::Absolute => absolute_time_string(timestamp),
    }
}

fn relative_time_string(now: i64, timestamp: i64) -> String {
    let delta = (now - timestamp).max(0);

    if delta < 60 {
        return format!("{delta}s ago");
    }

    let minutes = delta / 60;
    if minutes < 60 {
        return format!("{minutes}m ago");
    }

    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;

    if hours > 24 {
        let days = hours / 24;
        return format!("{days}d ago");
    }

    if remaining_minutes == 0 {
        return format!("{hours}h ago");
    }

    format!("{hours}h {remaining_minutes}m ago")
}

fn absolute_time_string(timestamp: i64) -> String {
    let datetime = match OffsetDateTime::from_unix_timestamp(timestamp) {
        Ok(value) => value,
        Err(_) => return "00:00".to_string(),
    };

    let localized = match UtcOffset::current_local_offset() {
        Ok(offset) => datetime.to_offset(offset),
        Err(_) => datetime,
    };

    match localized.format(&format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    )) {
        Ok(value) => value,
        Err(_) => "00:00".to_string(),
    }
}

fn session_state_label(state: SessionState) -> &'static str {
    match state {
        SessionState::Working => "working",
        SessionState::Blocked => "blocked",
        SessionState::Done => "done",
    }
}

fn session_state_color(state: SessionState, theme: &Theme) -> Color {
    match state {
        SessionState::Working => theme.info,
        SessionState::Blocked => theme.error,
        SessionState::Done => theme.success,
    }
}

fn confidence_color(confidence: i64, theme: &Theme) -> Color {
    let normalized = confidence.clamp(0, 100);
    let progress = normalized as f32 / 100.0;

    interpolate_color(theme.error, theme.success, progress)
}

fn interpolate_color(start: Color, end: Color, progress: f32) -> Color {
    let [start_r, start_g, start_b] = color_components(start);
    let [end_r, end_g, end_b] = color_components(end);
    let clamped = progress.clamp(0.0, 1.0);

    Color::Rgb(
        interpolate_channel(start_r, end_r, clamped),
        interpolate_channel(start_g, end_g, clamped),
        interpolate_channel(start_b, end_b, clamped),
    )
}

fn interpolate_channel(start: u8, end: u8, progress: f32) -> u8 {
    let start = f32::from(start);
    let end = f32::from(end);

    (start + ((end - start) * progress)).round() as u8
}

fn color_components(color: Color) -> [u8; 3] {
    match color {
        Color::Rgb(r, g, b) => [r, g, b],
        _ => [0, 0, 0],
    }
}

fn render_terminal_too_small(frame: &mut Frame, dimensions: &TerminalDimensions, theme: &Theme) {
    let message = format!(
        r#"
Terminal size too small:
  Width = {} Height = {}

Minimum dimensions needed:
  Width = {} Height = {}

Press (q/<ctrl+c>/<esc> to exit)
"#,
        dimensions.width, dimensions.height, MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT
    );

    let widget = Paragraph::new(message)
        .style(Style::default().fg(theme.foreground))
        .block(
            Block::bordered()
                .title(" crumbs ".bold().bg(theme.accent).fg(theme.background))
                .border_style(Style::default().fg(theme.accent)),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    frame.render_widget(widget, frame.area());
}
