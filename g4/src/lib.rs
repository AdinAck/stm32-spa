#![no_std]

pub mod common;
pub mod core;

#[cfg(chip_selected)]
pub use interrupts::g4::interrupt;
