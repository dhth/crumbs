# Intro

This tool is a command line application with two surfaces:
- a traditional CLI that coding agents can call to report progress
- a TUI that lets a human quickly view those agent sessions in one place

The main purpose of the tool is visibility. A user may have several coding
agents running at once across one or more projects, and they need a quick way
to check whether those agents are active, progressing, blocked, or finished.

The tool is not primarily an agent runner or orchestration system. It acts as a
lightweight status dashboard for agent activity.

## Initial Scope

The initial scope is simple:
- local-only operation
- read-only TUI
- a flat global list of agent sessions
- short updates emitted by agents at moments they consider meaningful

The TUI should help the user answer simple glanceable questions such as:
- which agents are currently active
- what each agent appears to be working on
- whether progress is being made
- whether any agent looks stuck, blocked, or done

## What Agents Report

Agents report lightweight progress updates through explicit CLI commands.

The tool is built around a few constraints:
- updates should be lightweight
- updates should be understandable at a glance
- the user does not need deep internal reasoning or full logs in the main view
- command responses should stay minimal so repeated agent calls do not pollute
  the agent's context window

In practice, the expected update shape is closer to a short status summary than
to raw chain-of-thought. A one-line update or short paragraph is enough.
