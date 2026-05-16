pub mod csr;

use crate::peripherals::rcc::enr::en;
use phm::{Composition, Peripheral};

use crate::devices::g4::vrefbuf::csr::csr;

pub fn vrefbuf(composition: &mut Composition, syscfgen: en::EnSchema) {
    let mut vrefbuf = composition.add_peripheral(Peripheral::new("vrefbuf", 0x4001_0030));

    vrefbuf.ontological_entitlements([[syscfgen.enabled]]);

    csr(&mut vrefbuf);
    // TODO: ccr
}
