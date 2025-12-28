use proto_hal_model::{Field, model::RegisterEntry};

use crate::dma::cmar;

pub fn ma32<'cx>(cmar: &mut RegisterEntry<'cx>, entitlements: cmar::Entitlements) {
    let mut ma32 = cmar
        .add_store_field(Field::new("ma32", 0, 30).docs(["Memory address for 32 bit transfers."]));

    ma32.ontological_entitlements([entitlements.msize.bits32]);
    ma32.write_entitlements([entitlements.en.disabled]);
}
