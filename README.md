# PicoUart

Bare-metal Rust ([embassy](https://embassy.dev)) experiments on the Raspberry Pi
Pico 2 (RP2350) that compare the **hardware UART** with a **UART transmitter
written in PIO assembly**. Both send the same bytes so you can inspect the
waveforms side by side on a logic analyser or scope.

## Wiring

| Signal            | GPIO | Physical pin |
|-------------------|------|--------------|
| Hardware UART0 TX | GP0  | 1            |
| Hardware UART0 RX | GP1  | 2 (unused)   |
| PIO UART TX       | GP2  | 4            |

Probe GP0 and GP2 (plus GND). Settings: 115200 baud, 8N1. The payload is `AU`
(0x41, 0x55); 0x55 = `01010101` gives an easy-to-read alternating pattern.

## Programs (`src/bin`)

| Binary           | What it does |
|------------------|--------------|
| `PicoUart`       | Baseline: sends `AU` every 10 ms on hardware UART0 (GP0). |
| `UartCompare`    | Sends `AU` on UART0 (GP0) and on embassy's stock `PioUartTx` (GP2). |
| `PioUartCustom`  | Sends `AU` on UART0 (GP0) and on our own PIO program (GP2). |
| `GpioClock`      | Toggles GP2 as a 1 kHz square wave (500 us half period); a timing reference. |

## Layout

```
src/
  lib.rs                  crate root, declares the modules below
  Board.rs                baud rate, payload, interval, PIO0 interrupt binding
  HwUart.rs               newHwUart(): UART0 setup on GP0/GP1
  CustomPioUartTx.rs      CustomPioUartTx: PIO state machine setup + write()
  PioPrograms/UartTx.pio  the PIO assembly program
  bin/                    the four example binaries (see above)
Makefile                  `make build` / `make flash`
memory.x                  RP2350 flash/RAM layout + boot-block sections
build.rs                  makes memory.x visible to the linker
.cargo/config.toml        target, defmt log level, `cargo run` flashing via picotool
Docs/PioUartExplained.md  how the PIO UART works, cycle by cycle
```

Naming follows the org convention: camelCase functions and variables,
UpperCamelCase types and files, UPPER_CASE constants.

## Build and run

Requirements: Rust with the `thumbv8m.main-none-eabihf` target
(`rustup target add thumbv8m.main-none-eabihf`) and
[picotool](https://github.com/raspberrypi/picotool).

Hold BOOTSEL while plugging in the board, then build and flash with `make`.
`build` and `flash` are separate targets; `flash` runs `build` first, so it is never stale:

```sh
make build                    # build the default binary (PioUartCustom)
make flash                    # build if missing or stale, then flash with picotool
```

Pick another binary with `BIN`:

```sh
make build BIN=UartCompare
make flash BIN=UartCompare
```

| Target       | Action |
|--------------|--------|
| `build`      | `cargo build --release --bin $(BIN)` |
| `build-all`  | build every binary |
| `flash`      | runs `build` first (no-op if up to date), then `picotool load -u -v -x -t elf` |
| `clean`      | `cargo clean` |
| `help`       | print usage |

`BIN` defaults to `PioUartCustom`; `PICOTOOL` overrides the picotool command.
Alternatively, `cargo run --release --bin PioUartCustom` builds and flashes in one go.

The cargo runner uses the same picotool command.
`DEFMT_LOG` defaults to `debug` in `.cargo/config.toml`. To use a debug probe
instead, change the runner to `probe-rs run --chip RP235x`.

## How it works

* **Hardware UART** (`HwUart.rs`): the RP2350 UART0 peripheral, configured by
  embassy with the shared baud rate. `blocking_write` pushes bytes into its FIFO.
* **PIO UART** (`CustomPioUartTx.rs` + `UartTx.pio`): a PIO state machine
  waits for a byte in its TX FIFO, then drives the pin itself: start bit, 8 data
  bits LSB first, stop bit. The state machine clock is divided down so every
  bit lasts exactly 8 PIO cycles. `write()` only queues bytes into the FIFO;
  the state machine transmits on its own with no CPU involvement.

For the cycle-by-cycle walkthrough and the clock divider maths, see
[Docs/PioUartExplained.md](Docs/PioUartExplained.md).
