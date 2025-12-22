use proto_hal_model::{Field, Variant, model::RegisterEntry};

use crate::dma::ccr::en;

pub fn htie<'cx>(ccr: &mut RegisterEntry<'cx>, en: en::Output) {
    let mut htie =
        ccr.add_store_field(Field::new("htie", 2, 1).docs(["Half transfer interrupt enable"]));

    htie.write_entitlements([en.disabled]);

    htie.add_variant(Variant::new("Disabled", 0));
    htie.add_variant(Variant::new("Enabled", 1));
}
