default: run

test:
	@cargo test

 ## Run all tests with coverage - `cargo install cargo-tarpaulin`
test-cov:
	@cargo tarpaulin

build:
	@make test && cargo build --release

run:
	@CARGO_INCREMENTAL=1 cargo fmt && make lint && cargo run

lint:
	@find . | grep '\.\/src\/.*\.rs$$' | xargs touch && cargo clippy --all-targets --workspace

lint-fix:
	@cargo fix

fmt:
	@cargo fmt

## Analyze for unsafe usage - `cargo install cargo-geiger`
analyze:
	@cargo geiger

release:
	@git tag -a ${V} -m "Release ${V}" && git push origin ${V}

delete-tag:
	@git tag -d ${V} && git push --delete origin ${V}