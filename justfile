VERSION := "latest"
IMG_NAME := "darkalex17/managarr"
IMAGE := "{{IMG_NAME}}:{{VERSION}}"


# List all recipes
default:
    @just --list

# Format all files
[group: 'style']
fmt:
    @cargo fmt --all

alias clippy := lint
# Run Clippy to inspect all files
[group: 'style']
lint:
    @cargo clippy --all

alias clippy-fix := lint-fix
# Automatically fix clippy issues where possible
[group: 'style']
lint-fix:
    @cargo fix

# Analyze the project for unsafe usage
[group: 'style']
@analyze:
    #!/usr/bin/env bash
    cargo geiger -h > /dev/null 2>&1 | cargo install cargo-geiger
    cargo geiger

# Run all tests
[group: 'test']
test:
    @cargo test --all

# Run all tests with coverage
[group:'test']
@test-cov:
    #!/usr/bin/env bash
    cargo tarpaulin -h > /dev/null 2>&1 || cargo install cargo-tarpaulin
    cargo tarpaulin

# Run all doc tests
[group: 'test']
doctest:
    @cargo test --all --doc

# Run all proptests
[group: 'test']
proptest:
    @cargo test proptest

alias test-snapshots := snapshot-tests
# Run all snapshot tests
[group: 'test']
snapshot-tests:
    @cargo test snapshot

alias review := snapshot-review
# Review snapshot test changes
[group: 'test']
@snapshot-review:
    #!/usr/bin/env bash
    cargo insta -h > /dev/null 2>&1 || cargo install cargo-insta
    cargo insta review

alias clean-orphaned-snapshots := snapshot-delete-unreferenced
# Delete any unreferenced snapshots
[group: 'test']
@snapshot-delete-unreferenced:
    #!/usr/bin/env bash
    cargo insta -h > /dev/null 2>&1 || cargo install cargo-insta
    cargo insta test --unreferenced=delete

# Build and run the binary for the current system
run:
    @cargo run

# Build the project for the current system architecture
[group: 'build']
[arg('build_type', pattern="debug|release")]
build build_type='debug':
    @cargo build {{ if build_type == "release" { "--release" } else { "" } }}

# Build the docker image
[group: 'build']
build-docker:
    @DOCKER_BUILDKIT=1 docker build --rm -t {{IMAGE}}
