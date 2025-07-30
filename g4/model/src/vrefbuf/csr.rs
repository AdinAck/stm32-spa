pub mod envr;
pub mod hiz;
pub mod vrr;
pub mod vrs;

use proto_hal_build::ir::structures::register::Register;

pub fn generate() -> Register {
    Register::new(
        "csr",
        0,
        [
            envr::generate(),
            hiz::generate(),
            vrr::generate(),
            vrs::generate(),
        ],
    )
}
