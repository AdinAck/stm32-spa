pub mod id;

use proto_hal_build::ir::structures::register::Register;

pub fn generate() -> Register {
    Register::new("idr", 0x10, (0..16).map(id::generate))
}
