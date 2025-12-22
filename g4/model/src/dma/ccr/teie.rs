use proto_hal_model::{Field, Variant, model::RegisterEntry};

use crate::dma::ccr::en;

pub fn teie<'cx>(ccr: &mut RegisterEntry<'cx>, en: en::Output) {
    let mut teie =
        ccr.add_store_field(Field::new("teie", 3, 1).docs(["Transfer error interrupt enable"]));

    teie.write_entitlements([en.disabled]);

    teie.add_variant(Variant::new("Disabled", 0));
    teie.add_variant(Variant::new("Enabled", 1));
}
