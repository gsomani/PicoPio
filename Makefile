# Build and flash the PicoPio example binaries.
#
#   make build                    build the default binary (release)
#   make flash                    build if needed, then flash with picotool
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

# Depends on build, so a missing or stale ELF is rebuilt first (cargo does
# nothing if it is up to date). Put the board in BOOTSEL mode (hold BOOTSEL
# while plugging in USB) before flashing.
flash: build
	$(PICOTOOL) load -u -v -x -t elf $(ELF)

clean:
	cargo clean
