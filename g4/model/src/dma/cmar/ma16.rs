use proto_hal_model::{Field, model::RegisterEntry};

use crate::dma::cmar;

pub fn ma16<'cx>(cmar: &mut RegisterEntry<'cx>, entitlements: cmar::Entitlements) {
    let mut ma16 = cmar
        .add_store_field(Field::new("ma16", 0, 31).docs(["Memory address for 16 bit transfers."]));

    ma16.ontological_entitlements([entitlements.msize.bits16]);
    ma16.write_entitlements([entitlements.en.disabled]);
}
