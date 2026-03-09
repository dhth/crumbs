Notes
---
- crumbs is a command line app that helps in surfacing progress from AI coding agents.
- The agents register sessions and report progress via the CLI; the user views the data via the TUI (not yet built)
- Behaviour documented in `docs/` (start with `docs/README.md` if needed).
- Command handling logic goes in `src/cmds`, domain types in `src/domain` and persistence logic in `src/persistence`
- Respect the clippy policy in `Cargo.toml`: `unwrap` and `expect` are denied in non-test code.

## Common Commands
- Prefer `just` over direct cargo commands.
- Build: `just build`
- Check: `just check`
- Test: `just test`
- Lint: `just lint`
- Format: `just fmt`
- Full local validation: `just all`
- Run CLI: `just run <args>`
