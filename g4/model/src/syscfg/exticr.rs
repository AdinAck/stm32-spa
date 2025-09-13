pub mod exti;

use proto_hal_build::ir::structures::register::Register;

#[derive(Clone, Copy)]
pub enum Instance {
    I1,
    I2,
    I3,
    I4,
}

impl Instance {
    fn ident(&self) -> String {
        match self {
            Instance::I1 => "exticr1",
            Instance::I2 => "exticr2",
            Instance::I3 => "exticr3",
            Instance::I4 => "exticr4",
        }
        .to_string()
    }

    fn offset(&self) -> u32 {
        match self {
            Instance::I1 => 0x08,
            Instance::I2 => 0x0c,
            Instance::I3 => 0x10,
            Instance::I4 => 0x14,
        }
    }
}

pub fn generate(instance: Instance) -> Register {
    Register::new(
        instance.ident(),
        instance.offset(),
        match instance {
            Instance::I1 => 0..4,
            Instance::I2 => 4..8,
            Instance::I3 => 8..12,
            Instance::I4 => 12..16,
        }
        .map(exti::generate),
    )
    .reset(0)
}
