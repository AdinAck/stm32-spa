pub mod ot;

use proto_hal_build::ir::structures::register::Register;

pub fn generate() -> Register {
    Register::new("otyper", 4, (0..16).map(ot::generate)).reset(0)
}
