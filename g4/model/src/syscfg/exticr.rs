pub mod exti;

use proto_hal_model::{Register, model::PeripheralEntry};

use crate::syscfg::exticr::exti::exti;

#[derive(Clone, Copy)]
pub enum Instance {
    I1,
    I2,
    I3,
    I4,
}

impl Instance {
    fn ident(&self) -> &str {
        match self {
            Instance::I1 => "exticr1",
            Instance::I2 => "exticr2",
            Instance::I3 => "exticr3",
            Instance::I4 => "exticr4",
        }
    }

    fn offset(&self) -> u32 {
        match self {
            Instance::I1 => 0x08,
            Instance::I2 => 0x0c,
            Instance::I3 => 0x10,
            Instance::I4 => 0x14,
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::I1, Self::I2, Self::I3, Self::I4].into_iter()
    }
}

pub fn exticr<'cx>(syscfg: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut exticr =
        syscfg.add_register(Register::new(instance.ident(), instance.offset()).reset(0));

    for i in match instance {
        Instance::I1 => 0..4,
        Instance::I2 => 4..8,
        Instance::I3 => 8..12,
        Instance::I4 => 12..16,
    } {
        exti(&mut exticr, i);
    }
}
