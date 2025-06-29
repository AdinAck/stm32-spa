pub mod od;

use proto_hal_build::ir::structures::register::Register;

pub fn generate() -> Register {
    Register::new("odr", 0x14, (0..16).map(od::generate))
}
