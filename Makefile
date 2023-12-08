.PHONY: run
run:
	RUST_LOG=warn cargo test day_0$(day) --release -- --nocapture

.PHONY: run
run-debug:
	RUST_LOG=debug cargo test day_0$(day) --release -- --nocapture
