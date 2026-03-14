---
name: crumbs
description: Report progress updates via the `crumbs` command line app when the user wants visibility into your work.
---

The user wants to stay updated on meaningful milestones as you work. You will leverage the command line app `crumbs` for this.

## Register

- Call `crumbs register <agent-name> <title>` once near the start of the task.
- Save the returned `session_id` and reuse it for all subsequent logs.

```bash
crumbs register <your-name> <session-title>
```

```json
{"session_id": 1}
```

## Log

- Call `crumbs log <session-id> <message>` for meaningful updates only.
- Keep messages short, concrete, and user-facing.
- Prefer milestone updates over micro-steps.
- Skip repetitive updates when nothing materially changed.
- Do not dump internal chain-of-thought.

```bash
crumbs log 1 "Added initial sessions table migration"
```

## States

- Use `--state working` when reporting active progress.
- Use `--state blocked` only when unable to proceed without outside change or intervention.
- Use `--state done` once, at the end, with a short final outcome.
- Omit `--state` when the session state should remain unchanged.

## Confidence

You can provide an optional flag `-c` to the log command to provide a value
between 0-100 that indicates your confidence level in finishing the task. Use
this flag when you're sure of the value.

## Wording

- Name session titles like git branch names (short, lowercase, dash separated)
- Keep crumb messages concise and similar to git commit messages

## What To Report

- Completed milestones
- Meaningful direction changes
- Blockers that may need user attention
- Final completion status
- If the user explicitly specifies what kind of updates to provide, keep that in mind

## What Not To Report

- File-by-file narration
- Every command you run
- Raw internal reasoning
- Duplicate status messages

## Failure Handling

- Do not let `crumbs` failures derail the main task.
- If registration fails, continue the main task unless the user has explicitly made progress reporting mandatory.
- If logging fails after registration, continue the main task and retry only if it is cheap and useful.
