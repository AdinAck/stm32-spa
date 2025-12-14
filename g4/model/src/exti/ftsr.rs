use proto_hal_model::{Register, model::PeripheralEntry};

use crate::exti::ftsr::ft::ft;

pub mod ft;

#[derive(Clone, Copy)]
pub enum Instance {
    I1,
}

impl Instance {
    fn ident(&self) -> String {
        match self {
            Instance::I1 => "ftsr1",
        }
        .to_string()
    }

    fn offset(&self) -> u32 {
        match self {
            Instance::I1 => 0x0c,
        }
    }
}

pub fn ftsr<'cx>(exti: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut ftsr = exti.add_register(
        Register::new(instance.ident(), instance.offset())
            .reset(0)
            .docs(["Falling Trigger Selection Register"]),
    );

    for i in match instance {
        Instance::I1 => 0..16,
    } {
        ft(&mut ftsr, i, i)
    }
}
