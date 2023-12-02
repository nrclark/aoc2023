default: build

PROGS := hello-world
PROJECT_NAME := $(notdir $(subst %/,%,$(abspath .)))

export RUSTFLAGS := -C target-cpu=native
CARGO_VERBS := clippy run build fmt

$(foreach x,$(CARGO_VERBS),cargo-$x): cargo-%:
	cargo $* --release

$(CARGO_VERBS): %:
	cargo $* --release

$(foreach x,$(PROGS),build-$(x)): build-%: build
	strip target/release/$*
	size target/release/$*


$(foreach x,$(PROGS),run-$(x)): run-%:
	cargo run --release --bin $*


cargo-clean clean:
	cargo clean
