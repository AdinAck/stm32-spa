pub mod exticr;

use peripherals::rcc::enr;
use phm::{Model, Peripheral};

use crate::syscfg::exticr::exticr;

pub fn syscfg(model: &mut Model, syscfgen: enr::Output) -> phm::Result<()> {
    let mut syscfg = model.add_peripheral(
        Peripheral::new("syscfg", 0x4001_0000).docs(["This peripheral is incomplete."]),
    );

    syscfg.ontological_entitlements([[syscfgen.enabled]])?;

    for instance in exticr::Instance::iter() {
        exticr(&mut syscfg, instance);
    }

    Ok(())
}
