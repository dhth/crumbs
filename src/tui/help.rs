pub const HELP_CONTENT: &str = r#"Keymaps

General

  <tab> / <S-tab>        move focus between panes
  ?                      open or close help view
  <Esc> / q              go back or quit
  <Ctrl+c>               quit immediately

Main View

  [                      select previous session
  ]                      select next session
  {                      select previous theme
  }                      select next theme
  t                      toggle time display

Sessions Pane

  j / down               select next session
  k / up                 select previous session
  J                      scroll crumbs down
  K                      scroll crumbs up

Crumbs Pane

  j / down               scroll crumbs down
  k / up                 scroll crumbs up
"#;

pub fn help_line_count() -> usize {
    HELP_CONTENT.lines().count()
}
