use proto_hal_model::{Register, model::PeripheralEntry};

use crate::exti::pr::pif::pif;

pub mod pif;

#[derive(Clone, Copy)]
pub enum Instance {
    I1,
}

impl Instance {
    fn ident(&self) -> String {
        match self {
            Instance::I1 => "pr1",
        }
        .to_string()
    }

    fn offset(&self) -> u32 {
        match self {
            Instance::I1 => 0x14,
        }
    }
}

pub fn pr<'cx>(exti: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut pr = exti.add_register(
        Register::new(instance.ident(), instance.offset()).docs(["Pending Register"]),
    );

    for i in match instance {
        Instance::I1 => 0..16,
    } {
        pif(&mut pr, i, i);
    }
}
