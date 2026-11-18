#![no_std]
#![no_main]
#![allow(non_snake_case)] // org convention: camelCase for functions and variables

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::uart::{PioUartTx, PioUartTxProgram};
use embassy_rp::uart::{Config, Uart};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const BAUD_RATE: u32 = 115_200;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    // Hardware UART0: TX = GP0 (pin 1), RX = GP1 (pin 2)
    let mut uartConfig = Config::default();
    uartConfig.baudrate = BAUD_RATE;
    let mut hwUart = Uart::new_blocking(peripherals.UART0, peripherals.PIN_0, peripherals.PIN_1, uartConfig);

    // PIO UART: state machine 0 on PIO0, TX = GP2 (pin 4)
    let Pio { mut common, sm0, .. } = Pio::new(peripherals.PIO0, Irqs);
    let txProgram = PioUartTxProgram::new(&mut common);
    let mut pioUart = PioUartTx::new(BAUD_RATE, &mut common, sm0, peripherals.PIN_2, &txProgram);

    loop {
        // Both sides send "AU" together: the PIO write only fills its TX FIFO,
        // then the hardware UART write starts right after.
        pioUart.write(b"AU").await; // 0x41, then 0x55
        hwUart.blocking_write(b"AU").unwrap();
        Timer::after_millis(10).await;
    }
}
