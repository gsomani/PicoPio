//! Hardware UART0 helper.

use embassy_rp::Peri;
use embassy_rp::peripherals::{PIN_0, PIN_1, UART0};
use embassy_rp::uart::{Config, Uart};
use embassy_rp::mode::Blocking;

use crate::Board::BAUD_RATE;

/// Creates a blocking UART0 at [`BAUD_RATE`] (8N1): TX = GP0, RX = GP1.
pub fn newHwUart<'d>(
    uart: Peri<'d, UART0>,
    txPin: Peri<'d, PIN_0>,
    rxPin: Peri<'d, PIN_1>,
) -> Uart<'d, Blocking> {
    let mut config = Config::default();
    config.baudrate = BAUD_RATE;
    Uart::new_blocking(uart, txPin, rxPin, config)
}
