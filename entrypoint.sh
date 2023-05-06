#!/bin/sh -l

echo "Running with coverage file $1"
echo "Comparing with branch $2"
echo "Using threshold $3%"
echo "Failing on total coverage below threshold: $4"

# Lower case argument 4 and check if it is true
fail_on_total_arg=$(echo "$4" | tr '[:upper:]' '[:lower:]')
if [ "$fail_on_total_arg" = "true" ]; then
    fail_on_total="--fail-on-total"
else
    fail_on_total=""
fi

/coverage_scope --coverage-file $GITHUB_WORKSPACE/$1 --branch $2 --threshold $3 --git-dir $GITHUB_WORKSPACE $fail_on_total
