MAKEFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
MAKEFILE_DIR  := $(dir $(MAKEFILE_PATH))

BIN := pass
LOCAL_BIN=$(HOME)/dots/personal/.local/bin/$(BIN)
RELEASE_BIN=$(MAKEFILE_DIR)/target/release/$(BIN)
DEBUG_BIN=$(MAKEFILE_DIR)/target/debug/$(BIN)

PASS := ./target/debug/pass
LINE := "------------------------------------------------------------"

run:
	cargo build
	$(PASS)

install:
	cargo install --path . --locked
	# make build
	# make load-bin

build:
	cargo build --release

# copies built binary to a path specified by $BIN
load-bin:
	@rm -f $(LOCAL_BIN)
	@cp $(RELEASE_BIN) $(LOCAL_BIN)
