# Build and flash the PicoUart example binaries.
#
#   make build                    build the default binary (release)
#   make flash                    flash the already-built binary with picotool
#   make build BIN=UartCompare    pick another binary
#   make build-all                build every binary
#   make clean
#
# Binaries: PicoUart, UartCompare, PioUartCustom, GpioClock

BIN     ?= PioUartCustom
TARGET  := thumbv8m.main-none-eabihf
ELF     := target/$(TARGET)/release/$(BIN)
PICOTOOL ?= picotool

.PHONY: build build-all flash clean help

help:
	@sed -n '2,9p' Makefile

build:
	cargo build --release --bin $(BIN)

build-all:
	cargo build --release --bins

# Does not rebuild: run `make build` first. Put the board in BOOTSEL mode
# (hold BOOTSEL while plugging in USB) before flashing.
flash:
	@test -f $(ELF) || { echo "$(ELF) not found: run 'make build BIN=$(BIN)' first"; exit 1; }
	$(PICOTOOL) load -u -v -x -t elf $(ELF)

clean:
	cargo clean
