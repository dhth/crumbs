# CLI

## Session Registration

The initial registration flow uses an explicit command:

```text
crumbs register <agent-name> <task-title>
```

This command creates a new session and returns a minimal JSON payload containing
only a tool-generated `session_id`. All later updates for that session must
include the returned `session_id`.

### Why This Shape

- Registration is explicit and easy for agents to follow.
- `agent_name` and `task_title` are required at the start of the session.
- The tool derives execution context on its own, including the current
  directory, project/repo identity, and git branch when available.
- A generated `session_id` avoids collisions when multiple agents work in the
  same repository, on the same branch, or even on the same task.

### Required Inputs

- `agent_name`: the reporting agent or harness name, such as `claude-code` or
  `codex`
- `title`: a short human-readable title describing the session

### Derived By The Tool

- current working directory
- project or repository name
- repository path
- git branch when available
- session start time
- unique `session_id`

These values are inferred from the invocation environment. Agents do not need
to discover or pass them during registration.

### Notes

- `task_title` is a required display label, not the session identifier.
- `session_id` is opaque and stable for the life of the session.
- No follow-up update should be accepted without prior registration.
- `register` returns the smallest useful payload so agent context is not
  polluted with metadata the caller does not need immediately.

## Logging Progress

After registration, agents report progress with an append-only log command:

```text
crumbs log <session-id> <message> [--state <working|blocked|done>] [--confidence <0-100>]
```

This command appends a new log event to the session history.

### Log Command Behavior

- `message` is required and should be a short human-readable update.
- `--state` is optional.
- If `--state` is omitted, the session keeps its previous state.
- If `--state` is provided, the session's current state changes to that value.
- `--confidence` is optional. It accepts an integer from 0 to 100 representing
  the agent's confidence in completing the overall session goal successfully.
- Every `log` call is append-only; past entries are not edited or replaced.
- Successful `log` calls produce no stdout output.
- Failed `log` calls should return a non-zero exit code and a short stderr
  message.
- Once a session reaches `done`, additional `log` calls are rejected.

### Why Log Is Silent

The tool is intended to be called repeatedly by agents. Silent success keeps
their context windows cleaner and makes progress reporting cheaper to use.

`register` is the exception because the caller needs the returned `session_id`.

### Example

```text
crumbs register claude-code "Refactor session state handling"
```

Response shape:

```json
{
  "session_id": 42
}
```

Example log calls:

```text
crumbs log 42 "Inspecting current state management code" --state working --confidence 80
crumbs log 42 "Need a decision on migration strategy" --state blocked --confidence 40
crumbs log 42 "Finished refactor and validated tests" --state done
```

## List All Sessions

```text
crumbs sessions
```

Returns a JSON array of all sessions ordered by most recently updated first.

### Output Format

```json
[
  {
    "id": 3,
    "agent_name": "opencode",
    "title": "Add sessions and list subcommands",
    "project_name": "crumbs",
    "path": "/Users/user/projects/crumbs",
    "branch": "feature-branch",
    "state": "working",
    "created_at": 1773052845,
    "updated_at": 1773052937
  }
]
```

## List Session Logs

```text
crumbs list <session-id>
```

Returns a JSON array of all log entries (crumbs) for the specified session, ordered chronologically (oldest first).

### Output Format

```json
[
  {
    "id": 1,
    "session_id": 3,
    "message": "Added sessions and list subcommands",
    "state": null,
    "confidence": null,
    "created_at": 1773052856
  },
  {
    "id": 2,
    "session_id": 3,
    "message": "Implementation complete",
    "state": "done",
    "confidence": 95,
    "created_at": 1773052937
  }
]
```

## Open The TUI

```text
crumbs tui [--theme <name>]
```

Opens the terminal UI for viewing sessions and crumbs interactively.

## Overriding Database Path

All commands accept an optional global database path override:

```text
crumbs [--db-path <path-to-file.db>] <command>
```
