pub mod exticr;

use proto_hal_build::ir::structures::peripheral::Peripheral;

pub fn generate() -> Peripheral {
    Peripheral::new(
        "syscfg",
        0x4001_0000,
        [
            exticr::generate(exticr::Instance::I1),
            exticr::generate(exticr::Instance::I2),
            exticr::generate(exticr::Instance::I3),
            exticr::generate(exticr::Instance::I4),
        ],
    )
    .docs(["This peripheral is incomplete."])
}
