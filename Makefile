# Einstieg für Entwickler und Agenten. Siehe CLAUDE.md §3.
export RUSTUP_TOOLCHAIN ?= stable
CRATE := contextra-storage
T ?=

.PHONY: check fmt-check clippy test test-target test-summary layering loom doc metrics

check:
	cargo check --workspace --all-targets

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets --message-format=short

test:
	cargo test --workspace --no-fail-fast

test-target:
	@test -n "$(T)" || { echo "Aufruf: make test-target T=<target>"; exit 2; }
	cargo test -p $(CRATE) --test $(T)

test-summary:
	cargo test --workspace --no-fail-fast 2>&1 | grep -E '^(test result|     Running|error(\[|:)|test .* FAILED)'

layering:
	cargo test -p $(CRATE) --test layering

loom:
	RUSTFLAGS="--cfg loom" CARGO_TARGET_DIR=target/loom cargo test -p $(CRATE) --test loom

doc:
	RUSTDOCFLAGS="-D warnings" cargo doc -p $(CRATE) --no-deps

metrics:
	@sh scripts/metrics.sh
