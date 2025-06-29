pub mod afsel;

use proto_hal_build::ir::structures::register::Register;

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

pub fn generate(instance: Instance) -> Register {
    Register::new(
        instance.ident(),
        instance.offset(),
        match instance {
            Instance::L => 0..8,
            Instance::H => 8..16,
        }
        .map(afsel::generate),
    )
}
