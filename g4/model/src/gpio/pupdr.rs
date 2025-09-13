pub mod pupd;

use proto_hal_build::ir::structures::register::Register;

use crate::gpio::Instance;

pub fn generate(instance: Instance) -> Register {
    Register::new("pupdr", 0xc, (0..16).map(pupd::generate)).reset(match instance {
        Instance::A => 0x6400_0000,
        Instance::B => 0x100,
        _ => 0,
    })
}
