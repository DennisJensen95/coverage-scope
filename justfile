lint: 
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-targets --all-features

test-coverage:
    cargo tarpaulin --all-features --out html

ci: lint test
    cargo check --all-targets --all-features

test-local:
    cargo tarpaulin --out Xml
    cargo run -- --coverage-file cobertura.xml --branch main --threshold-change 90 --threshold-total 90

test-docker-actions-env:
    cargo tarpaulin --out Xml
    docker build -t coverage-scope -f Dockerfile.build .
    docker run --rm -it -v $(pwd):/repo -e "GITHUB_WORKSPACE=/repo" coverage-scope cobertura.xml main 80 90
    