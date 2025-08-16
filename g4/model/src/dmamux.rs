pub mod cfr;
pub mod csr;
pub mod cxcr;
pub mod rgcfr;
pub mod rgsr;
pub mod rgxcr;

use proto_hal_build::ir::structures::{entitlement::Entitlement, peripheral::Peripheral};

pub fn generate(instances: u8, channels: u8) -> Peripheral {
    Peripheral::new(
        "dmamux",
        0x4002_0800,
        (0..instances * channels)
            .map(cxcr::generate)
            .chain([csr::generate(), cfr::generate()])
            .chain((0..4).map(rgxcr::generate))
            .chain([rgsr::generate(), rgcfr::generate()]),
    )
    .entitlements([Entitlement::to("rcc::ahb1enr::dmamux1en::Enabled")])
}
