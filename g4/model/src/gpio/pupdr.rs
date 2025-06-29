pub mod pupd;

use proto_hal_build::ir::structures::register::Register;

use crate::gpio::Instance;

pub fn generate(instance: Instance) -> Register {
    Register::new("pupdr", 0xc, (0..16).map(|x| pupd::generate(x, instance)))
}
