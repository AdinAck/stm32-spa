mod csr;
mod rdata;
mod wdata;

use peripherals::rcc::enr;
use phm::{ModelBuilder, Peripheral};

use csr::csr;
use rdata::rdata;
use wdata::wdata;

pub fn cordic(model: &mut ModelBuilder, cordicen: enr::Output) {
    let mut cordic = model.add_peripheral(Peripheral::new("cordic", 0x4002_0c00));

    cordic.ontological_entitlements([[cordicen.enabled]]);

    let csr = csr(&mut cordic);

    wdata(
        &mut cordic,
        wdata::Entitlements {
            argsize_q15: csr.argsize.q15,
            argsize_q31: csr.argsize.q31,
            nargs_one: csr.nargs.one,
        },
    );

    rdata(
        &mut cordic,
        rdata::Entitlements {
            ressize_q15: csr.ressize.q15,
            ressize_q31: csr.ressize.q31,
            nres_one: csr.nres.one,
        },
    );
}
