# Session Lifecycle

## Core Session States

The lifecycle model is small. A session can be in one of
three states:

- `working`
- `blocked`
- `done`

## Why The State Model Is Small

The tool's main job is to help a user quickly scan many agent sessions.
Because of that, the state model stays coarse and readable.

Detailed sub-phases such as "prototyping", "refining", or "thinking through
alternatives" should not become formal states. Those are better represented as
short update messages attached to a session while it remains in the `working`
state.

This keeps the dashboard simple:
- formal state answers whether the session is progressing, stuck, or finished
- update text explains what the agent is doing right now

## Meaning Of Each State

- `working`: the agent is making meaningful forward progress
- `blocked`: the agent cannot proceed without outside change or intervention
- `done`: the session has finished its task

## Notes

- `blocked` should be reserved for cases where the agent is effectively stopped,
  not merely exploring or reconsidering an approach.
- Fine-grained work descriptions belong in updates, not in the state enum.
- The tool may later derive additional signals such as staleness, but those are
  separate from the core lifecycle state model.
