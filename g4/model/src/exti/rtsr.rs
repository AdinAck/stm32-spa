pub mod rt;

use proto_hal_model::{Register, model::PeripheralEntry};

use crate::exti::rtsr::rt::rt;

#[derive(Clone, Copy)]
pub enum Instance {
    I1,
}

impl Instance {
    fn ident(&self) -> &str {
        match self {
            Instance::I1 => "rtsr1",
        }
    }

    fn offset(&self) -> u32 {
        match self {
            Instance::I1 => 0x08,
        }
    }
}

pub fn rtsr<'cx>(exti: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut rtsr = exti.add_register(
        Register::new(instance.ident(), instance.offset())
            .reset(0)
            .docs(["Rising Trigger Selection Register"]),
    );

    for i in match instance {
        Instance::I1 => 0..16,
    } {
        rt(&mut rtsr, i, i)
    }
}
