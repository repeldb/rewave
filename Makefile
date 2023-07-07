.PHONY: help, copyright

help:
	$(info Rewave Makefile)
	$(info )
	$(info Consider to use 'cargo' instead)
	$(info )
	
	@grep '^[[:alnum:]_-]*:.* ##' $(MAKEFILE_LIST) \
		| sort | awk 'BEGIN {FS=":.* ## "}; {printf "%-25s %s\n", $$1, $$2};'


run-server: # Run server example
	@RUST_BACKTRACE=full RUST_LOG=info cargo run --release --example server

run-client:# Run server client
	@RUST_BACKTRACE=full RUST_LOG=info cargo run --release --example client

clean:
	@cargo clean

test:
	@RUST_BACKTRACE=full RUST_LOG=info cargo test

check-setup:
	@type rustup >/dev/null 2>&1 || (echo "Install rustup first. To install, run Run 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh'" >&2 ; exit 1)
	@type rustc >/dev/null 2>&1 || (echo "Install rustc first. To install, run Run 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh'" >&2 ; exit 1)

lint:
	@cargo fix
	@cargo clippy --fix -Z unstable-options
	@cargo clippy --all-targets --all-features -- -D warnings

format:
	@cargo fmt
	@echo "Code formatted successfully"

doc:
	@cargo doc --target-dir docs

copyright: 
	@find . -iname "*.rs" -exec bash -c "if ! grep -q Copyright "{}"; then cat COPYRIGHT {} > {}.new && mv {}.new {} ; fi" \; 
	@echo "Copyright notice added"
