<!-- Badge ci build ci.yml -->
![Build Status](https://github.com/DennisJensen95/coverage-scope/actions/workflows/ci.yml/badge.svg)
![Code coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/DennisJensen95/2b7862c80c14d562c8659e1283543190/raw/coverage-scope.json)

# coverage-scope

The coverage-scope package ensures that a `cobertura.xml` coverage format file
meets a minimum coverage threshold. It is specially designed to check if the
coverage of changed files in a pull request meets a minimum threshold,
preventing a decrease in coverage. The package also checks the total coverage of
the project.

## Supported languages

The package currently only supports the following coding languages:

- Python
- Rust

These are the only languages that have been tested. However, the logic should be
applicable for other languages. (Currently we filter on extension `.py` and
`.rs` - so it will not work unless we change the code)

## Usage

IMPORTANT: When checking out the repository, you need to set the fetch `depth`
to `0` the `actions/checkout@v3` action.

```yaml
- uses: actions/checkout@v3
  with:
    fetch-depth: 0
```

To use the GitHub actions for coverage-scope, you need to provide the
`cobertura` coverage file in xml format, the branch you want to compare against
when measuring changed files coverage, and the minimum threshold for both the
total coverage and the changed files coverage.

```yaml
- uses: dennisjensen95/coverage-scope@v0.1.4
  with: 
    coverage-filepath: cobertura.xml
    branch: main
    threshold-total: 80
    threshold-change: 90
```

I made this because I could not find a GitHub action that did this. I wanted
specifically to allow pull requests to not have a total specific coverage and
then only check if the changed files in the pull request met a minimum
threshold. This is useful if you have a large project with a lot of legacy code,
or you join a team that has not been testing, that you do not want to have to fix
coverage for. This way you can ensure that new code meets a minimum threshold.
