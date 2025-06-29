pub mod ospeed;

use proto_hal_build::ir::structures::register::Register;

use crate::gpio::Instance;

pub fn generate(instance: Instance) -> Register {
    Register::new("ospeedr", 8, (0..16).map(|x| ospeed::generate(x, instance)))
}
