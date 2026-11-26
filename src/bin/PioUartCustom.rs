//! Our own PIO UART transmitter, sent alongside the hardware UART for comparison.
//!
//!   GP0 (pin 1): hardware UART0 TX
//!   GP2 (pin 4): CustomPioUartTx on PIO0 / state machine 0

#![no_std]
#![no_main]
#![allow(non_snake_case)] // org convention: camelCase for functions and variables

use embassy_executor::Spawner;
use embassy_rp::pio::Pio;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use PicoPio::Board::{Irqs, BAUD_RATE, SEND_INTERVAL_MS, TEST_MESSAGE};
use PicoPio::CustomPioUartTx::CustomPioUartTx;
use PicoPio::HwUart::newHwUart;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let mut hwUart = newHwUart(peripherals.UART0, peripherals.PIN_0, peripherals.PIN_1);

    let Pio { mut common, sm0, .. } = Pio::new(peripherals.PIO0, Irqs);
    let mut pioUart = CustomPioUartTx::new(&mut common, sm0, peripherals.PIN_2, BAUD_RATE);

    loop {
        pioUart.write(TEST_MESSAGE).await;
        hwUart.blocking_write(TEST_MESSAGE).unwrap();
        Timer::after_millis(SEND_INTERVAL_MS).await;
    }
}
