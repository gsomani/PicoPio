#![no_std]
#![no_main]
#![allow(non_snake_case)] // org convention: camelCase for functions and variables

use embassy_executor::Spawner;
use embassy_rp::uart::{Config, Uart};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const BAUD_RATE: u32 = 115_200;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let mut uartConfig = Config::default();
    uartConfig.baudrate = BAUD_RATE;

    // UART0: TX = GP0 (pin 1), RX = GP1 (pin 2)
    let mut uart = Uart::new_blocking(peripherals.UART0, peripherals.PIN_0, peripherals.PIN_1, uartConfig);

    loop {
        uart.blocking_write(b"AU").unwrap(); // 0x41, then 0x55 (alternating bits)
        Timer::after_millis(10).await;
    }
}
