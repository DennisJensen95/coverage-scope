lint: 
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-targets --all-features

test-coverage:
    cargo tarpaulin --all-features --out html

ci: lint test
    cargo check --all-targets --all-features
    