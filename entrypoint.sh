#!/bin/sh -l

echo "Running with coverage file $1"
echo "Comparing with branch $2"
echo "Using threshold $3%"

/coverage_scope --coverage-file $GITHUB_WORKSPACE/$1 --branch $2 --threshold $3 --git-dir $GITHUB_WORKSPACE
