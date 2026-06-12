set dotenv-load := false

compose := "docker compose"

default:
    @just --list

build-image:
    {{compose}} build

shell:
    {{compose}} run --rm dev bash

proto-generate:
    {{compose}} run --rm dev just _proto-generate

test:
    {{compose}} run --rm dev just _test

ci:
    {{compose}} run --rm dev just _ci

_proto-generate:
    npm install
    npm run proto:generate

_test:
    cargo test
    npm test

_ci: _proto-generate _test
