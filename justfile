default:
    just --list

test:
    cargo test --features tokio
    cargo test --doc

lint:
    cargo fmt -- --check
    cargo clippy --features tokio --all-targets -- -D warnings
    cargo clippy --lib --target wasm32-unknown-unknown -- -D warnings

check: lint test
