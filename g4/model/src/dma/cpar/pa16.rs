use proto_hal_model::{Field, model::RegisterEntry};

use crate::dma::cpar;

pub fn pa16<'cx>(cpar: &mut RegisterEntry<'cx>, entitlements: cpar::Entitlements) {
    let mut pa16 = cpar.add_store_field(
        Field::new("pa16", 0, 31).docs(["Peripheral address for 16 bit transfers."]),
    );

    pa16.ontological_entitlements([entitlements.psize.bits16]);
    pa16.write_entitlements([entitlements.en.disabled]);
}
