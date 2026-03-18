pub mod csr;

use peripherals::rcc::enr;
use phm::{Model, Peripheral};

use crate::vrefbuf::csr::csr;

pub fn vrefbuf(model: &mut Model, syscfgen: enr::Output) -> phm::Result<()> {
    let mut vrefbuf = model.add_peripheral(Peripheral::new("vrefbuf", 0x4001_0030));

    vrefbuf.ontological_entitlements([[syscfgen.enabled]])?;

    csr(&mut vrefbuf);
    // TODO: ccr

    Ok(())
}
