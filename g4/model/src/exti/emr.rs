pub mod em;

use proto_hal_model::{Register, model::PeripheralEntry};

use crate::exti::emr::em::em;

#[derive(Clone, Copy)]
pub enum Instance {
    I1,
}

impl Instance {
    fn ident(&self) -> &str {
        match self {
            Instance::I1 => "emr1",
        }
    }

    fn offset(&self) -> u32 {
        match self {
            Instance::I1 => 0x04,
        }
    }
}

pub fn emr<'cx>(exti: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut emr = exti.add_register(
        Register::new(instance.ident(), instance.offset())
            .reset(0)
            .docs(["Event Mask Register"]),
    );

    for i in match instance {
        Instance::I1 => 0..16,
    } {
        em(&mut emr, i, i);
    }
}
