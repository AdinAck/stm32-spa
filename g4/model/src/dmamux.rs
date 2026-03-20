use peripherals::rcc::enr;
use phm::{Composition, Peripheral};

use crate::dmamux::{ccr::ccr, cfr::cfr, csr::csr, rgcfr::rgcfr, rgcr::rgcr, rgsr::rgsr};

pub mod ccr;
pub mod cfr;
pub mod csr;
pub mod rgcfr;
pub mod rgcr;
pub mod rgsr;

pub fn dmamux(composition: &mut Composition, instances: u8, channels: u8, dmamux1en: enr::Output) {
    let mut dmamux = composition.add_peripheral(Peripheral::new("dmamux", 0x4002_0800));

    dmamux.ontological_entitlements([[dmamux1en.enabled]]);

    for i in 0..instances * channels {
        ccr(&mut dmamux, i);
    }

    csr(&mut dmamux);
    cfr(&mut dmamux);

    for i in 0..4 {
        rgcr(&mut dmamux, i);
    }

    rgsr(&mut dmamux);
    rgcfr(&mut dmamux);
}
