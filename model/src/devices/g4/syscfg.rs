pub mod exticr;

use crate::peripherals::rcc::enr::en;
use phm::{Composition, Peripheral};

use crate::devices::g4::syscfg::exticr::exticr;

pub fn syscfg(composition: &mut Composition, syscfgen: en::EnSchema) {
    let mut syscfg = composition.add_peripheral(
        Peripheral::new("syscfg", 0x4001_0000).docs(["This peripheral is incomplete."]),
    );

    syscfg.ontological_entitlements([[syscfgen.enabled]]);

    for instance in exticr::Instance::iter() {
        exticr(&mut syscfg, instance);
    }
}
