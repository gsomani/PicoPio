# The PIO UART transmitter explained

This document walks through `src/PioPrograms/UartTx.pio` and
`src/CustomPioUartTx.rs`.

## UART frame (8N1)

```
idle(high) | start(0) | d0 d1 d2 d3 d4 d5 d6 d7 | stop(1) | idle(high)
```

Data goes out least-significant bit first. At 115200 baud one bit lasts
1/115200 = 8.68 us.

## The PIO program

```
.wrap_target
    pull block          ; wait for a byte in the TX FIFO, move it to the OSR
    set pins, 0 [6]     ; start bit: pin low, then 6 delay cycles
    set x, 7            ; bit counter = 7, pin unchanged
bitLoop:
    out pins, 1 [6]     ; shift 1 bit from OSR to the pin, 6 delay cycles
    jmp x-- bitLoop     ; repeat while x != 0 (8 passes, x = 7..0)
    set pins, 1 [6]     ; stop bit: pin high, 6 delay cycles
.wrap                   ; jump back to `pull block`
```

Each instruction takes 1 cycle plus its `[delay]`. Every bit lasts 8 cycles:

| Step      | Instructions                    | Cycles |
|-----------|---------------------------------|--------|
| Start bit | `set pins, 0 [6]` + `set x, 7`  | 7 + 1 = 8 |
| Data bit  | `out pins, 1 [6]` + `jmp x--`   | 7 + 1 = 8 (x8) |
| Stop bit  | `set pins, 1 [6]` + wrap `pull` | 7 + 1 = 8 |

The wrap from the last instruction back to `pull block` costs no cycle. The
`pull` that follows takes one cycle, which completes the stop bit. Because the
`pull` blocks while the FIFO is empty, the pin stays high (idle) until the next
byte arrives. A byte pushed while the stop bit is still being sent follows
back-to-back with no gap.

## Setting up the state machine (`CustomPioUartTx::new`)

| Setting | Why |
|---------|-----|
| `set_set_pins` / `set_out_pins` = TX pin | `set pins` (start and stop bits) and `out pins` (data bits) must both drive the same pin. |
| `shift_out.direction = Right` | The OSR shifts out LSB first, as UART requires. |
| `shift_out.auto_fill = false` | The program pulls explicitly with `pull block`. |
| `fifo_join = TxOnly` | Merges the RX FIFO into the TX FIFO: 8 entries instead of 4. |
| `set_pins(High)` + `set_pin_dirs(Out)` | The pin is high and an output before the program starts, so there is no false start bit. |

### Clock divider

The state machine must run at `8 x baud` cycles per second, so:

```
divider = sysclk / (CYCLES_PER_BIT * baud)
```

PIO dividers are 16.8 fixed point (8 fractional bits), so the code computes
`sysclk * 256 / (8 * baud)`. At a 150 MHz system clock and 115200 baud the
divider is about 162.76. The fractional part makes the baud rate accurate to a
small fraction of a percent.

`CYCLES_PER_BIT` in `CustomPioUartTx.rs` must be changed together with the
delays in the `.pio` file.

## Using it

```rust
let Pio { mut common, sm0, .. } = Pio::new(peripherals.PIO0, Irqs);
let mut pioUart = CustomPioUartTx::new(&mut common, sm0, peripherals.PIN_2, BAUD_RATE);
pioUart.write(b"AU").await;
```

`write` waits only while the 8-entry FIFO is full. It returns when bytes are
queued, not when they have left the pin.

## Comparing with the hardware UART

Run `PioUartCustom` (or `UartCompare` for embassy's stock PIO UART) and capture
GP0 and GP2 together. The frames should look identical: same start bit, same
bit timing, same stop bit. Small differences in the bit period come from the
different clock dividers (the hardware UART uses its own divider).
