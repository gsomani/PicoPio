//! Shared building blocks for the PicoUart example binaries.
//!
//! * [`Board`]: constants, pin map and interrupt bindings shared by every binary.
//! * [`HwUart`]: helper to bring up the RP2350 hardware UART0.
//! * [`CustomPioUartTx`]: a UART transmitter built from our own PIO program.
//!
//! See `README.md` for wiring and a walkthrough of each binary.

#![no_std]
#![allow(non_snake_case)] // org convention: camelCase for functions and variables

pub mod Board;
pub mod CustomPioUartTx;
pub mod HwUart;
