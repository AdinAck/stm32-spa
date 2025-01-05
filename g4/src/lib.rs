#![no_std]

pub mod common;

#[cfg(chip_selected)]
pub use interrupts::g4::interrupt;
