#!/bin/sh -l

echo "Running with coverage file $1"
echo "Comparing with branch $2"
echo "Using threshold for change: $3%"
echo "using threshold for total: $4"

/coverage_scope --coverage-file $GITHUB_WORKSPACE/$1 --branch $2 --threshold-change $3 --threshold-total $4 --git-dir $GITHUB_WORKSPACE
