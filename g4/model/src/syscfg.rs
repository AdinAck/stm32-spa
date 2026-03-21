pub mod exticr;

use peripherals::rcc::enr;
use phm::{Composition, Peripheral};

use crate::syscfg::exticr::exticr;

pub fn syscfg(composition: &mut Composition, syscfgen: enr::Output) {
    let mut syscfg = composition.add_peripheral(
        Peripheral::new("syscfg", 0x4001_0000).docs(["This peripheral is incomplete."]),
    );

    syscfg.ontological_entitlements([[syscfgen.enabled]]);

    for instance in exticr::Instance::iter() {
        exticr(&mut syscfg, instance);
    }
}
