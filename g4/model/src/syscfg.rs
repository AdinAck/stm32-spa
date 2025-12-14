pub mod exticr;

use proto_hal_model::{Entitlement, Model, Peripheral};

use crate::syscfg::exticr::exticr;

pub fn syscfg(model: &mut Model, syscfgen: Entitlement) {
    let mut syscfg = model.add_peripheral(
        Peripheral::new("syscfg", 0x4001_0000).docs(["This peripheral is incomplete."]),
    );

    syscfg.ontological_entitlements([syscfgen]);

    for instance in exticr::Instance::iter() {
        exticr(&mut syscfg, instance);
    }
}
