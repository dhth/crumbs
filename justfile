alias a := all
alias b := build
alias c := check
alias f := fmt
alias fc := fmt-check
alias i := install
alias l := lint
alias lf := lint-fix
alias r := run
alias t := test

export DATABASE_URL := 'sqlite://db/crumbs.db'

@default:
    just --choose

build:
    cargo build

check:
    cargo check

sqlx-check:
    cargo sqlx prepare --check

sqlx-prepare:
    cargo sqlx prepare

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

install:
    cargo install --path .

lint:
    cargo clippy

lint-fix:
    cargo clippy --fix  --allow-dirty --allow-staged

publish-dry:
    cargo publish --dry-run --allow-dirty

run *ARGS:
    cargo run -- {{ ARGS }}

test:
    cargo test

@all:
    just check
    just fmt
    just lint
    just sqlx-check
    just test
