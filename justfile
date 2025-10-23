default:
    just --list

test:
    cargo test
    cargo test --doc

lint:
    cargo fmt -- --check
    cargo clippy --all-targets -- -D warnings

check: lint test
