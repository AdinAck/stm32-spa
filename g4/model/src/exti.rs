pub mod emr;
pub mod ftsr;
pub mod imr;
pub mod pr;
pub mod rtsr;

use proto_hal_build::ir::structures::peripheral::Peripheral;

pub fn generate() -> Peripheral {
    Peripheral::new(
        "exti",
        0x4001_0400,
        [
            imr::generate(imr::Instance::I1),
            emr::generate(emr::Instance::I1),
            rtsr::generate(rtsr::Instance::I1),
            ftsr::generate(ftsr::Instance::I1),
            pr::generate(pr::Instance::I1),
        ],
    )
    .docs(["This peripheral is incomplete."])
}
