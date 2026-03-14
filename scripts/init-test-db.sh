#!/usr/bin/env bash
set -euo pipefail

# Initialize test database with sample sessions and crumbs
# Usage: ./scripts/init-test-db.sh <path-to-crumbs-binary>
# Example: ./scripts/init-test-db.sh ./target/debug/crumbs

CRUMBS_BIN="${1:-./target/debug/crumbs}"
DB_PATH="/var/tmp/crumbs/test.db"

run_crumbs() {
	"$CRUMBS_BIN" --db-path "$DB_PATH" "$@"
}

# Session 1: 5 crumbs (opencode agent)
echo "Creating session 1: implement-auth"
SESSION1=$(run_crumbs register opencode 'implement-auth' | jq -r '.session_id')
run_crumbs log "$SESSION1" 'Created user authentication middleware' --state working -c 80
run_crumbs log "$SESSION1" 'Implemented JWT token generation' -c 75
run_crumbs log "$SESSION1" 'Waiting for review on password validation logic' --state blocked -c 60
run_crumbs log "$SESSION1" 'Review approved, merging auth changes' --state working -c 90
run_crumbs log "$SESSION1" 'Authentication feature complete' --state done -c 100

# Session 2: 5 crumbs (claude agent)
echo "Creating session 2: refactor-db-layer"
SESSION2=$(run_crumbs register claude 'refactor-db-layer' | jq -r '.session_id')
run_crumbs log "$SESSION2" 'Analyzing current database schema' --state working -c 85
run_crumbs log "$SESSION2" 'Identified N+1 query issues in user endpoints' -c 90
run_crumbs log "$SESSION2" 'Refactored query builder for better performance' -c 70
run_crumbs log "$SESSION2" 'Tests passing, preparing for deployment' -c 95
run_crumbs log "$SESSION2" 'Database layer refactoring merged' --state done -c 100

# Session 3: 5 crumbs (copilot agent)
echo "Creating session 3: add-dark-mode"
SESSION3=$(run_crumbs register copilot 'add-dark-mode' | jq -r '.session_id')
run_crumbs log "$SESSION3" 'Designing color token system' --state working -c 60
run_crumbs log "$SESSION3" 'Created CSS variables for themes' -c 75
run_crumbs log "$SESSION3" 'Stuck on accessibility contrast requirements' --state blocked -c 40
run_crumbs log "$SESSION3" 'Resolved contrast issues with design team' --state working -c 80
run_crumbs log "$SESSION3" 'Dark mode toggle implemented and tested' --state done -c 100

# Session 4: 60 crumbs (opencode agent)
echo "Creating session 4: migrate-to-sqlx"
SESSION4=$(run_crumbs register opencode 'migrate-to-sqlx' | jq -r '.session_id')
run_crumbs log "$SESSION4" 'Researching sqlx vs diesel async capabilities' --state working -c 60
run_crumbs log "$SESSION4" 'Prototyping sqlx compile-time query validation' -c 70
run_crumbs log "$SESSION4" 'Drafting migration proposal for team review' -c 75
run_crumbs log "$SESSION4" 'Team approved sqlx migration plan' -c 80
run_crumbs log "$SESSION4" 'Phase 1: Migration planning complete' --state done -c 85
run_crumbs log "$SESSION4" 'Setting up sqlx with compile-time query checks' --state working -c 78
run_crumbs log "$SESSION4" 'Configuring connection pool for production loads' -c 80
run_crumbs log "$SESSION4" 'Migrated user repository to sqlx' -c 82
run_crumbs log "$SESSION4" 'Migrated session repository to sqlx' -c 85
run_crumbs log "$SESSION4" 'Migrated crumb repository to sqlx' -c 87
run_crumbs log "$SESSION4" 'Refactoring error handling for sqlx' -c 88
run_crumbs log "$SESSION4" 'All repository migrations complete' --state done -c 90
run_crumbs log "$SESSION4" 'Running integration test suite' --state working -c 85
run_crumbs log "$SESSION4" 'Unit tests passing for all repositories' -c 87
run_crumbs log "$SESSION4" 'Load testing connection pool under stress' -c 83
run_crumbs log "$SESSION4" 'Pool timeouts occurring at high concurrency' --state blocked -c 45
run_crumbs log "$SESSION4" 'Analyzing connection pool metrics' -c 50
run_crumbs log "$SESSION4" 'Tuning pool size and timeout settings' --state working -c 65
run_crumbs log "$SESSION4" 'All tests passing with new configuration' -c 88
run_crumbs log "$SESSION4" 'Core migration validated and stable' --state done -c 90
run_crumbs log "$SESSION4" 'Benchmarking against baseline metrics' --state working -c 75
run_crumbs log "$SESSION4" 'Query performance improved 30%' -c 85
run_crumbs log "$SESSION4" 'Memory usage reduced 20%' -c 88
run_crumbs log "$SESSION4" 'Performance goals achieved' --state done -c 92
run_crumbs log "$SESSION4" 'Writing migration runbook for ops team' --state working -c 80
run_crumbs log "$SESSION4" 'Documenting query patterns for developers' -c 82
run_crumbs log "$SESSION4" 'Adding sqlx prepare check to CI pipeline' -c 85
run_crumbs log "$SESSION4" 'Code review feedback addressed' -c 90
run_crumbs log "$SESSION4" 'PR approved and merged to main' --state done -c 95
run_crumbs log "$SESSION4" 'Preparing staging environment migration' --state working -c 88
run_crumbs log "$SESSION4" 'Creating database backup before migration' -c 90
run_crumbs log "$SESSION4" 'Testing rollback procedures' -c 85
run_crumbs log "$SESSION4" 'Staging migration successful' -c 92
run_crumbs log "$SESSION4" 'Verifying data integrity post-migration' -c 93
run_crumbs log "$SESSION4" 'Monitoring staging metrics for 24 hours' -c 90
run_crumbs log "$SESSION4" 'Staging validation complete' --state done -c 95
run_crumbs log "$SESSION4" 'Preparing production deployment window' --state working -c 93
run_crumbs log "$SESSION4" 'Large result sets causing memory issues in prod-like data' --state blocked -c 50
run_crumbs log "$SESSION4" 'Implementing streaming for large queries' --state working -c 70
run_crumbs log "$SESSION4" 'Load testing streaming with production dataset' -c 80
run_crumbs log "$SESSION4" 'Memory usage stable under load' -c 90
run_crumbs log "$SESSION4" 'Blocker resolved, production ready' --state done -c 95
run_crumbs log "$SESSION4" 'Executing production migration' --state working -c 92
run_crumbs log "$SESSION4" 'Database migration completed successfully' -c 95
run_crumbs log "$SESSION4" 'Verifying application health checks' -c 93
run_crumbs log "$SESSION4" 'Monitoring error rates post-deployment' -c 90
run_crumbs log "$SESSION4" 'Error rates stable, performance nominal' -c 95
run_crumbs log "$SESSION4" 'Production deployment successful' --state done -c 98
run_crumbs log "$SESSION4" 'Removing diesel dependencies from codebase' --state working -c 90
run_crumbs log "$SESSION4" 'Updating Cargo.toml and lock files' -c 91
run_crumbs log "$SESSION4" 'Cleaning up old migration files' -c 92
run_crumbs log "$SESSION4" 'Migration cleanup complete' --state done -c 100
run_crumbs log "$SESSION4" 'User requested team retrospective documentation' --state working -c 85
run_crumbs log "$SESSION4" 'Scheduling team retrospective session' -c 87
run_crumbs log "$SESSION4" 'Conducting team retrospective session' -c 90
run_crumbs log "$SESSION4" 'Retrospective completed' --state done -c 95
run_crumbs log "$SESSION4" 'User requested wiki update with best practices' --state working -c 88
run_crumbs log "$SESSION4" 'Documenting sqlx patterns in team wiki' -c 92
run_crumbs log "$SESSION4" 'Publishing best practices guide' -c 95
run_crumbs log "$SESSION4" 'Wiki documentation complete' --state done -c 100

# Session 5: 15 crumbs (claude agent)
echo "Creating session 5: implement-tui"
SESSION5=$(run_crumbs register claude 'implement-tui' | jq -r '.session_id')
run_crumbs log "$SESSION5" 'Researching ratatui for terminal interface' --state working -c 60
run_crumbs log "$SESSION5" 'Set up basic TUI app structure' -c 70
run_crumbs log "$SESSION5" 'Implemented session list view' -c 75
run_crumbs log "$SESSION5" 'Added keyboard navigation' -c 80
run_crumbs log "$SESSION5" 'Designing crumb detail view' -c 65
run_crumbs log "$SESSION5" 'Async event handling blocking TUI updates' --state blocked -c 45
run_crumbs log "$SESSION5" 'Resolved async event loop integration' --state working -c 70
run_crumbs log "$SESSION5" 'TUI rendering smoothly with async events' -c 80
run_crumbs log "$SESSION5" 'Added color themes support' -c 85
run_crumbs log "$SESSION5" 'Testing on different terminal emulators' -c 80
run_crumbs log "$SESSION5" 'Fixed rendering issue on small screens' -c 90
run_crumbs log "$SESSION5" 'Added help panel with key bindings' -c 88
run_crumbs log "$SESSION5" 'Polished UI transitions' -c 92
run_crumbs log "$SESSION5" 'Final testing complete' -c 95
run_crumbs log "$SESSION5" 'TUI implementation merged' --state done -c 100

echo ""
echo "Test database initialized successfully!"
echo "Sessions created: $SESSION1, $SESSION2, $SESSION3, $SESSION4, $SESSION5"
echo ""
echo "Run 'just run-test tui' to explore the data"
