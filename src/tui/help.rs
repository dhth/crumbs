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
  g                      select first session
  G                      select last session
  x                      archive selected session
  J                      scroll crumbs down
  K                      scroll crumbs up

Crumbs Pane

  j / down               scroll down
  k / up                 scroll up
  g                      scroll to the top
  G                      scroll to the bottom
"#;

pub fn help_line_count() -> usize {
    HELP_CONTENT.lines().count()
}
