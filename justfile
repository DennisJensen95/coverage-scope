lint: 
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-targets --all-features

test-coverage:
    cargo tarpaulin --all-features --out html

ci: lint test
    cargo check --all-targets --all-features

test-docker-actions-env:
    cargo tarpaulin --out Xml
    docker build -t coverage-scope .
    docker run --rm -it -v $(pwd):/repo -e "GITHUB_WORKSPACE=/repo" coverage-scope cobertura.xml main 0
    