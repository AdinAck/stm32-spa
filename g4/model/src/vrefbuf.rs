pub mod csr;

use peripherals::rcc::enr;
use phm::{Composition, Peripheral};

use crate::vrefbuf::csr::csr;

pub fn vrefbuf(composition: &mut Composition, syscfgen: enr::Output) {
    let mut vrefbuf = composition.add_peripheral(Peripheral::new("vrefbuf", 0x4001_0030));

    vrefbuf.ontological_entitlements([[syscfgen.enabled]]);

    csr(&mut vrefbuf);
    // TODO: ccr
}
