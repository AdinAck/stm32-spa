pub mod ospeed;

use proto_hal_build::ir::structures::register::Register;

use crate::gpio::Instance;

pub fn generate(instance: Instance) -> Register {
    Register::new("ospeedr", 8, (0..16).map(ospeed::generate)).reset(match instance {
        Instance::A => 0x08,
        Instance::B => 0x0c00_0000,
        _ => 0,
    })
}
