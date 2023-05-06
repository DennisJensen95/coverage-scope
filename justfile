lint: 
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-targets --all-features

ci: lint test
    cargo check --all-targets --all-features
    