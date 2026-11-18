//! A hand-written PIO UART transmitter (8N1), compared against the hardware UART.
//!
//!   GP0 (pin 1): hardware UART0 TX
//!   GP2 (pin 4): our own PIO program on PIO0 / state machine 0

#![no_std]
#![no_main]
#![allow(non_snake_case)] // org convention: camelCase for functions and variables

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::clk_sys_freq;
use embassy_rp::gpio::Level;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::program::pio_file;
use embassy_rp::pio::{Config as PioConfig, Direction, FifoJoin, InterruptHandler, Pio, ShiftDirection};
use embassy_rp::uart::{Config as UartConfig, Uart};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const BAUD_RATE: u32 = 115_200;
const CYCLES_PER_BIT: u32 = 8; // must match the timing in the PIO program below

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());



    // Hardware UART0: TX = GP0 , RX = GP1
    let mut uartConfig = UartConfig::default();
    uartConfig.baudrate = BAUD_RATE;
    let mut hwUart = Uart::new_blocking(peripherals.UART0, peripherals.PIN_0, peripherals.PIN_1, uartConfig);







    let Pio { mut common, mut sm0, .. } = Pio::new(peripherals.PIO0, Irqs);
    let txProgram = pio_file!("src/PioPrograms/UartTx.pio", select_program("uartTx"));
    let txProgram2 = pio_file!("src/PioPrograms/UartTx2.pio", select_program("uartTx2"));

    let loadedProgram = common.load_program(&txProgram.program);

    let txPin = common.make_pio_pin(peripherals.PIN_2);  // PIN_2 (GP2)

    let mut pioConfig = PioConfig::default();

    pioConfig.use_program(&loadedProgram, &[]); // no side-set pins
    pioConfig.set_set_pins(&[&txPin]); // used by `set pins`  
    pioConfig.set_out_pins(&[&txPin]); // used by `out pins`
    pioConfig.shift_out.direction = ShiftDirection::Right; // LSB first, like UART
    pioConfig.shift_out.auto_fill = false; // we use an explicit `pull`
    pioConfig.fifo_join = FifoJoin::TxOnly; // 8-deep TX FIFO
    


    // The state machine runs at sysclk / divider. We want 8 cycles per bit.
    // The divider is a 16.8 fixed-point number, hence the * 256.
    pioConfig.clock_divider_x256 = ((clk_sys_freq() as u64 * 256) / (CYCLES_PER_BIT * BAUD_RATE) as u64) as u32;
    sm0.set_config(&pioConfig);
    sm0.set_pins(Level::High, &[&txPin]); // idle high before the program starts
    sm0.set_pin_dirs(Direction::Out, &[&txPin]);
    sm0.set_enable(true);

    loop {
        // The PIO state machine takes each byte from its FIFO and shifts it out on GP2.
        sm0.tx().wait_push(b'A' as u32).await; // 0x41
        sm0.tx().wait_push(b'U' as u32).await; // 0x55
        hwUart.blocking_write(b"AU").unwrap();
        Timer::after_millis(10).await;
    }
}
