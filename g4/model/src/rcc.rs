pub mod ahbenr;

use proto_hal_build::ir::structures::peripheral::Peripheral;

pub fn generate() -> Peripheral {
    Peripheral::new(
        "rcc",
        0x4002_1000,
        [
            ahbenr::generate(ahbenr::Instance::I1),
            ahbenr::generate(ahbenr::Instance::I2),
        ],
    )
}
