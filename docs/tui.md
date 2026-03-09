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
| > [working]          | 14:03  [working]  Inspecting state handling                 |
|   claude-code        | 14:05             Comparing two refactor approaches         |
|   Refactor sessio... | 14:07  [blocked]  Need a decision on migration strategy     |
|   12s ago            | 14:10  [working]  Resuming after migration decision         |
|                      | 14:15             Tightening session list width             |
|   [blocked]          | 14:20  [blocked]  Waiting on final pane layout choice       |
|   codex              | 14:24  [working]  Continuing after layout was chosen        |
|   Design persiste... | 14:28             Moving metadata below crumb timeline      |
|   1m ago             | 14:31             Pressure-testing readability              |
|                      | 14:35  [done   ]  Documented default TUI layout             |
|   [working]          |                                                             |
|   open-code          |                                                             |
|   Prototype event... |                                                             |
|   3m ago             |                                                             |
|                      |                                                             |
|   [done]             |-------------------------------------------------------------|
|   claude-code        | Metadata                                                    |
|   Fix cargo fmt f... | task       Refactor session state handling                  |
|   6m ago             | agent      claude-code                                      |
|                      | project    crumbs                                           |
|   [working]          | path       /Users/.../projects/crumbs                       |
|   codex              | branch     feature/tui-status                               |
|   Add session his... | state      done                                             |
|   7m ago             | started    14:03:11                                         |
|                      | last crumb 12s ago                                          |
+------------------------------------------------------------------------------------+
| status bar (user messages go here)                                                 |
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
- timestamps for each crumb
- state when a crumb includes a state transition

The crumb pane receives most of the available space in the layout.

State changes should be visible in the crumb list when they occur. They are
useful because they make transitions into `blocked` or `done` obvious in the
history without requiring the user to infer them from message text alone.

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

### Status Bar And Footer

The layout also includes two bottom rows:
- a status bar for user-facing messages, similar to a Vim status line
- a footer row for showing various indicators to the user

These rows reserve space for feedback and future UI signals without changing
the main pane layout.

## Deferred Alternate Views

These view ideas are deferred for now:

- feed-first view with a global live crumb stream across all sessions
- column board grouped by `working`, `blocked`, and `done`
- a footer-style metadata layout where metadata spans the entire width beneath
  the crumb pane

These may still become useful secondary views later, but they are not the
default opening experience.
