#!make
VERSION  := latest
IMG_NAME := darkalex17/managarr
IMAGE    := ${IMG_NAME}:${VERSION}

default: run

.PHONY: test test-cov build run lint lint-fix fmt analyze sonar release delete-tag

test:
	@cargo test --all

## Run all tests with coverage - `cargo install cargo-tarpaulin`
test-cov:
	@cargo tarpaulin

build: test
	@cargo build --release

docker:
	@DOCKER_BUILDKIT=1 docker build --rm -t ${IMAGE} .

run:
	@CARGO_INCREMENTAL=1 cargo fmt && make lint && cargo run

lint:
	@find . | grep '\.\/src\/.*\.rs$$' | xargs touch && CARGO_INCREMENTAL=0 cargo clippy --all-targets --workspace

lint-fix:
	@cargo fix

fmt:
	@cargo fmt

minimal-versions:
	@cargo +nightly update -Zdirect-minimal-versions

## Analyze for unsafe usage - `cargo install cargo-geiger`
analyze:
	@cargo geiger

release:
	@git tag -a ${V} -m "Release ${V}" && git push origin ${V}

delete-tag:
	@git tag -d ${V} && git push --delete origin ${V}

