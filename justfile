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

push-docker VERSION_TAG:
    docker buildx build --platform linux/amd64 -t coverage-scope-amd64 -f Dockerfile.build .
    docker tag coverage-scope-amd64 dennisjensen95/coverage-scope:${VERSION_TAG}
    docker push dennisjensen95/coverage-scope:${VERSION_TAG}
    