//! Baseline: transmit on the RP2350 hardware UART0 only.
//!
//!   GP0 (pin 1): UART0 TX

#![no_std]
#![no_main]
#![allow(non_snake_case)] // org convention: camelCase for functions and variables

use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use PicoUart::Board::{SEND_INTERVAL_MS, TEST_MESSAGE};
use PicoUart::HwUart::newHwUart;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let mut uart = newHwUart(peripherals.UART0, peripherals.PIN_0, peripherals.PIN_1);

    loop {
        uart.blocking_write(TEST_MESSAGE).unwrap();
        Timer::after_millis(SEND_INTERVAL_MS).await;
    }
}
