pub mod csr;

use proto_hal_build::ir::structures::{entitlement::Entitlement, peripheral::Peripheral};

pub fn generate() -> Peripheral {
    Peripheral::new(
        "vrefbuf",
        0x4001_0030,
        [csr::generate() /* TODO: CCR register. */],
    )
    .entitlements([Entitlement::to("rcc::apb2enr::syscfgen::Enabled")])
}
