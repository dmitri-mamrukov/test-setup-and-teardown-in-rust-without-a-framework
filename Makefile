coverage:
	cargo tarpaulin --verbose

analyze-code:
	cargo clippy

format:
	cargo fmt

test: analyze-code format
	cargo test -- --nocapture --test-threads=1

run:
	cargo run

clean:
	cargo clean
