.PHONY: all build

all: build

build:
	CARGO_TARGET_DIR="$(CURDIR)/target" cargo build --locked
