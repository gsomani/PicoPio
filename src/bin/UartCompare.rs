//! Hardware UART vs. the stock embassy PIO UART, transmitting the same bytes.
//!
//!   GP0 (pin 1): hardware UART0 TX
//!   GP2 (pin 4): embassy `PioUartTx` on PIO0 / state machine 0

#![no_std]
#![no_main]
#![allow(non_snake_case)] // org convention: camelCase for functions and variables

use embassy_executor::Spawner;
use embassy_rp::pio::Pio;
use embassy_rp::pio_programs::uart::{PioUartTx, PioUartTxProgram};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

use PicoPio::Board::{Irqs, BAUD_RATE, SEND_INTERVAL_MS, TEST_MESSAGE};
use PicoPio::HwUart::newHwUart;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let mut hwUart = newHwUart(peripherals.UART0, peripherals.PIN_0, peripherals.PIN_1);

    let Pio { mut common, sm0, .. } = Pio::new(peripherals.PIO0, Irqs);
    let txProgram = PioUartTxProgram::new(&mut common);
    let mut pioUart = PioUartTx::new(BAUD_RATE, &mut common, sm0, peripherals.PIN_2, &txProgram);

    loop {
        // The PIO write only fills its TX FIFO; the hardware UART write follows
        // immediately, so both lines transmit at nearly the same time.
        pioUart.write(TEST_MESSAGE).await;
        hwUart.blocking_write(TEST_MESSAGE).unwrap();
        Timer::after_millis(SEND_INTERVAL_MS).await;
    }
}
