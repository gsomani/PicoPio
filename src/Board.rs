//! Constants and interrupt bindings shared by all binaries.
//!
//! Pin map (Pico 2):
//!
//! | Signal              | GPIO | Physical pin |
//! |---------------------|------|--------------|
//! | Hardware UART0 TX   | GP0  | 1            |
//! | Hardware UART0 RX   | GP1  | 2            |
//! | PIO UART / clock TX | GP2  | 4            |

use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::InterruptHandler;

/// Baud rate used by every UART in this project.
pub const BAUD_RATE: u32 = 115_200;

/// Test payload: 0x41 then 0x55. 0x55 is `01010101`, so it shows up as a clean
/// alternating waveform on a scope or logic analyser.
pub const TEST_MESSAGE: &[u8] = b"AU";

/// Pause between two transmissions, in milliseconds.
pub const SEND_INTERVAL_MS: u64 = 10;

bind_interrupts!(
    /// Interrupt bindings required by the PIO0 driver.
    pub struct Irqs {
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);
