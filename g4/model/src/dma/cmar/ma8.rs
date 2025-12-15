use proto_hal_model::{Field, model::RegisterEntry};

use crate::dma::cmar;

pub fn ma8<'cx>(cmar: &mut RegisterEntry<'cx>, entitlements: cmar::Entitlements) {
    let mut ma8 = cmar
        .add_store_field(Field::new("ma8", 0, 32).docs(["Memory address for 8 bit transfers."]));

    ma8.ontological_entitlements([entitlements.msize.bits8]);
    ma8.write_entitlements([entitlements.en.disabled]);
}
