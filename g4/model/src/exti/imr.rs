pub mod im;

use proto_hal_model::{Register, model::PeripheralEntry};

use crate::exti::imr::im::im;

#[derive(Clone, Copy)]
pub enum Instance {
    I1,
}

impl Instance {
    fn ident(&self) -> &str {
        match self {
            Instance::I1 => "imr1",
        }
    }

    fn offset(&self) -> u32 {
        match self {
            Instance::I1 => 0x00,
        }
    }
}

pub fn imr<'cx>(exti: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut imr = exti.add_register(
        Register::new(instance.ident(), instance.offset())
            .reset(0)
            .docs(["Interrupt Mask Register"]),
    );

    for i in match instance {
        Instance::I1 => 0..16,
    } {
        im(&mut imr, i, i);
    }
}
