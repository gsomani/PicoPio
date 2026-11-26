//! A UART transmitter (8N1) driven by our own PIO program.
//!
//! The PIO program lives in `src/PioPrograms/UartTx.pio`; read its comments
//! together with this file. Each bit lasts [`CYCLES_PER_BIT`] PIO clock cycles,
//! so the state machine clock is `sysclk / (CYCLES_PER_BIT * baud)`.

use embassy_rp::Peri;
use embassy_rp::clocks::clk_sys_freq;
use embassy_rp::gpio::Level;
use embassy_rp::pio::program::pio_file;
use embassy_rp::pio::{
    Common, Config, Direction, FifoJoin, Instance, PioPin, ShiftDirection, StateMachine,
};

/// PIO cycles per UART bit. Must match the delays in `UartTx.pio`.
const CYCLES_PER_BIT: u32 = 8;

/// UART transmitter on one PIO state machine.
pub struct CustomPioUartTx<'d, PIO: Instance, const SM: usize> {
    sm: StateMachine<'d, PIO, SM>,
}

impl<'d, PIO: Instance, const SM: usize> CustomPioUartTx<'d, PIO, SM> {
    /// Loads the program, configures the state machine and starts it.
    ///
    /// The line idles high until the first byte is written.
    pub fn new(
        common: &mut Common<'d, PIO>,
        mut sm: StateMachine<'d, PIO, SM>,
        pin: Peri<'d, impl PioPin>,
        baudRate: u32,
    ) -> Self {
        let program = pio_file!("src/PioPrograms/UartTx.pio", select_program("uartTx"));
        let loadedProgram = common.load_program(&program.program);
        let txPin = common.make_pio_pin(pin);

        let mut config = Config::default();
        config.use_program(&loadedProgram, &[]); // no side-set pins
        config.set_set_pins(&[&txPin]); // used by `set pins` (start/stop bits)
        config.set_out_pins(&[&txPin]); // used by `out pins` (data bits)
        config.shift_out.direction = ShiftDirection::Right; // LSB first, like UART
        config.shift_out.auto_fill = false; // the program uses an explicit `pull`
        config.fifo_join = FifoJoin::TxOnly; // 8-deep TX FIFO

        // The divider is a 16.8 fixed-point number, hence the * 256.
        let sysClockHz = clk_sys_freq() as u64;
        config.clock_divider_x256 = ((sysClockHz * 256) / (CYCLES_PER_BIT * baudRate) as u64) as u32;

        sm.set_config(&config);
        sm.set_pins(Level::High, &[&txPin]); // idle high before the program starts
        sm.set_pin_dirs(Direction::Out, &[&txPin]);
        sm.set_enable(true);

        Self { sm }
    }

    /// Queues `data` into the TX FIFO, waiting whenever the FIFO is full.
    ///
    /// Returns once the bytes are queued, not once they are on the wire.
    pub async fn write(&mut self, data: &[u8]) {
        for &byte in data {
            self.sm.tx().wait_push(byte as u32).await;
        }
    }
}
