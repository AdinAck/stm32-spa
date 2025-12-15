use proto_hal_model::{Field, Variant, model::RegisterEntry};

use crate::dma::ccr::en;

pub fn tcie<'cx>(ccr: &mut RegisterEntry<'cx>, en: en::Output) {
    let mut tcie =
        ccr.add_store_field(Field::new("tcie", 1, 1).docs(["Transfer complete interrupt enable"]));

    tcie.write_entitlements([en.disabled]);

    tcie.add_variant(Variant::new("Disabled", 0));
    tcie.add_variant(Variant::new("Enabled", 1));
}
