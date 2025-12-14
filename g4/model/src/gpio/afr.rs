use proto_hal_model::{Register, model::PeripheralEntry};

use crate::gpio::afr::afsel::afsel;

pub mod afsel;

#[derive(Clone, Copy)]
pub enum Instance {
    L,
    H,
}

impl Instance {
    pub fn ident(&self) -> String {
        match self {
            Instance::L => "afrl",
            Instance::H => "afrh",
        }
        .to_string()
    }

    pub fn offset(&self) -> u32 {
        match self {
            Instance::L => 0x20,
            Instance::H => 0x24,
        }
    }
}

pub fn afr<'cx>(gpio: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut afr = gpio.add_register(Register::new(instance.ident(), instance.offset()).reset(0));

    for i in match instance {
        Instance::L => 0..8,
        Instance::H => 8..16,
    } {
        afsel(&mut afr, i);
    }
}
