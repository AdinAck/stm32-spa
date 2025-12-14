pub mod csr;

use proto_hal_model::{Entitlement, Model, Peripheral};

use crate::vrefbuf::csr::csr;

pub fn vrefbuf(model: &mut Model, syscfgen: Entitlement) {
    let mut vrefbuf = model.add_peripheral(Peripheral::new("vrefbuf", 0x4001_0030));

    vrefbuf.ontological_entitlements([syscfgen]);

    csr(&mut vrefbuf);
    // TODO: ccr
}
