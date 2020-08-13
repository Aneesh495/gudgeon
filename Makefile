.PHONY: all build test verify sim bench lint clean help

CARGO ?= cargo
PYTHON ?= python3

all: build

build:
	$(CARGO) build --release

test:
	$(CARGO) test --all
	$(CARGO) run -- test

verify:
	$(CARGO) run -- verify

sim:
	$(CARGO) run -- sim

bench:
	$(CARGO) run -- bench

lint:
	$(PYTHON) tools/check_no_em_dash.py
	$(CARGO) clippy --all-targets -- -D warnings

clean:
	$(CARGO) clean

help:
	@echo "Gudgeon Autonomous Surface Vessel Navigation & Control Engine"
	@echo "Targets:"
	@echo "  build   : Compile release binary"
	@echo "  test    : Run Rust and LOLCODE test suites"
	@echo "  verify  : Execute 6-gate qualification executive"
	@echo "  sim     : Run 3-DOF vessel real-time simulation"
	@echo "  bench   : Run instruction and sensor benchmarks"
	@echo "  lint    : Enforce zero em-dash and run clippy"
	@echo "  clean   : Remove build artifacts"
