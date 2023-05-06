<!-- Badge ci build ci.yml -->
![Build Status](https://github.com/DennisJensen95/coverage-scope/actions/workflows/ci.yml/badge.svg)
![Code coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/DennisJensen95/2b7862c80c14d562c8659e1283543190/raw/coverage-scope.json)

# coverage-scope

The coverage-scope package provides a simple way to check if a cobertura.xml
coverage format file meets a minimum coverage threshold. Specifically, it was
designed to check on the PR commit diff if the coverage of the changed files
meets a minimum threshold. That way on each PR we do never decrease a desired
threshold of coverage.

## Usage
