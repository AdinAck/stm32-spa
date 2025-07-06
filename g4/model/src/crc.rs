pub mod dr;

use proto_hal_build::ir::structures::{entitlement::Entitlement, peripheral::Peripheral};

pub fn generate() -> Peripheral {
    Peripheral::new("crc", 0x4002_3000, [dr::generate()])
        .entitlements([Entitlement::to("rcc::ahb1enr::crcen::Enabled")])
}
