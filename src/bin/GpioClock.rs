#![no_std]
#![no_main]
#![allow(non_snake_case)] // org convention: camelCase for functions and variables

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const HALF_PERIOD_US: u64 = 500;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // GP2 (physical pin 4) as a push-pull output, starting low
    let mut clockPin = Output::new(peripherals.PIN_2, Level::Low);

    loop {
        clockPin.set_high();
        Timer::after_micros(HALF_PERIOD_US).await;
        clockPin.set_low();
        Timer::after_micros(HALF_PERIOD_US).await;
    }
}
