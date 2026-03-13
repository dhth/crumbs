# TUI

## Default Opening View

The default TUI opening view is a List + Detail TeamView.

The default layout is crumb-first:
- a thin session list on the left for quick scanning
- a large crumb timeline for the selected session in the upper right
- session metadata below the crumb timeline

This layout is intended to help the user do two things at once:
- keep awareness of all active sessions
- read the progress history of one selected session without leaving the main
  screen

## Why This Layout Was Chosen

- The tool's main value is the crumbs, so they dominate the screen.
- The session list is still important, but it mainly supports navigation and
  glanceable state awareness.
- Metadata matters, but it is secondary to the activity stream and should follow
  it visually.

## Default Layout Mock

```text
+------------------------------------------------------------------------------------+
| Sessions (7)         | Crumbs: Refactor session state handling                     |
|----------------------|-------------------------------------------------------------|
| > Refactor sessio... | [working]  C80  Inspecting state handling                   |
|   opencode           |            C80  Comparing two refactor approaches           |
|   [working]          | [blocked]  C40  Need a decision on migration strategy       |
|   12s ago            | [working]  C60  Resuming after migration decision           |
|                      |            C70  Tightening session list width               |
|   Design persiste... | [blocked]  C30  Waiting on final pane layout choice         |
|   pi                 | [working]  C75  Continuing after layout was chosen          |
|   [done]             |            C85  Moving metadata below crumb timeline        |
|   1m ago             |            C90  Pressure-testing readability                |
|                      | [done]          Documented default TUI layout               |
|                      |                                                             |
|                      |                                                             |
|                      |                                                             |
|                      |                                                             |
|                      |                                                             |
|                      |-------------------------------------------------------------|
|                      | Metadata                                                    |
|                      | task       Refactor session state handling                  |
|                      | agent      claude-code                                      |
|                      | project    crumbs                                           |
|                      | path       /Users/.../projects/crumbs                       |
|                      | branch     feature/tui-status                               |
|                      | state      done                                             |
|                      | started    14:03:11                                         |
|                      | last crumb 12s ago                                          |
+------------------------------------------------------------------------------------+
| crumbs                                                                             |
+------------------------------------------------------------------------------------+
```

## Pane Responsibilities

### Left Session List

The left pane is intentionally narrow and stays glanceable.

Each row emphasizes:
- session state
- agent name
- a truncated task title
- relative recency of the latest crumb

The left pane avoids trying to show too much detail. Its job is to help
the user choose which session to inspect.

Sessions default to a recent-activity ordering so the most recently
updated work stays near the top.

### Upper-Right Crumb Pane

The upper-right pane is the main reading surface.

It shows:
- the selected session's crumb history
- state when a crumb includes a state transition
- confidence when a crumb includes a confidence value, displayed as `C<value>`

The crumb pane receives most of the available space in the layout.

State changes should be visible in the crumb list when they occur. They are
useful because they make transitions into `blocked` or `done` obvious in the
history without requiring the user to infer them from message text alone.

Confidence is displayed with color interpolation from the error color (low
confidence) to the success color (high confidence).

### Lower-Right Metadata Pane

The lower-right pane shows supporting session context, such as:
- task title
- agent name
- project name
- path
- branch when available
- current state
- started time
- last crumb recency

This pane is important for disambiguation, but it is not the primary reading
surface.

### Footer

The layout includes a footer row at the bottom that shows:
- the application name
- user-facing messages such as theme change confirmations or error notices
- status indicators

The footer reserves space for feedback without changing the main pane layout.

## Deferred Alternate Views

These view ideas are deferred for now:

- feed-first view with a global live crumb stream across all sessions
- column board grouped by `working`, `blocked`, and `done`
- a footer-style metadata layout where metadata spans the entire width beneath
  the crumb pane

These may still become useful secondary views later, but they are not the
default opening experience.

## Key Bindings

Read `src/tui/help.rs`.

## Themes

The TUI ships with nine built-in color themes. The starting theme can be set
with the `--theme` flag when launching the TUI. Themes can also be cycled at
runtime with `{` and `}`.

## Time Display

Timestamps in the session list and metadata pane support two display modes:

- **relative** (default) — shows how long ago an event occurred, such as
  `12s ago` or `3m ago`
- **absolute** — shows the full date and time, such as `2026-03-13 14:03:11`

Pressing `t` toggles between the two modes.

## Session Markers

When sessions are refreshed, the session list marks entries that have changed
since the last refresh:

- `[new]` — a session that was not present in the previous list
- `[updated]` — a session whose `updated_at` timestamp has advanced

Selecting a session clears its marker.

## Minimum Terminal Dimensions

The TUI requires a minimum terminal size of 80 columns by 30 rows. If the
terminal is smaller, a message is shown asking the user to resize. Normal
rendering resumes once the terminal meets the minimum dimensions.
