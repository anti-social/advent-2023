.PHONY: run
run:
	RUST_LOG=warn cargo test day_0$(day) -- --nocapture

.PHONY: run
run-debug:
	RUST_LOG=debug cargo test day_0$(day) --jobs 1 -- --nocapture
