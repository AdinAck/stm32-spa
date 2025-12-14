pub mod ospeed;

use proto_hal_model::{Register, model::PeripheralEntry};

use crate::gpio::{Instance, ospeedr::ospeed::ospeed};

pub fn ospeedr<'cx>(gpio: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut ospeedr = gpio.add_register(Register::new("ospeedr", 8).reset(match instance {
        Instance::A => 0x08,
        Instance::B => 0x0c00_0000,
        _ => 0,
    }));

    for i in 0..16 {
        ospeed(&mut ospeedr, i);
    }
}
