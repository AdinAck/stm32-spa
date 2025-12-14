pub mod pupd;

use proto_hal_model::{Register, model::PeripheralEntry};

use crate::gpio::{Instance, pupdr::pupd::pupd};

pub fn pupdr<'cx>(gpio: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut pupdr = gpio.add_register(Register::new("pudpr", 0xc).reset(match instance {
        Instance::A => 0x6400_0000,
        Instance::B => 0x100,
        _ => 0,
    }));

    for i in 0..16 {
        pupd(&mut pupdr, i);
    }
}
