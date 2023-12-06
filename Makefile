.PHONY: run
run:
	RUST_LOG=warn cargo test day_0$(day) -- --nocapture
