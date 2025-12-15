use proto_hal_model::{Field, Variant, model::RegisterEntry};

use crate::dma::ccr::en;

pub fn pinc<'cx>(ccr: &mut RegisterEntry<'cx>, en: en::Output) {
    let mut pinc =
        ccr.add_store_field(Field::new("pinc", 6, 1).docs(["Peripheral increment mode"]));

    pinc.write_entitlements([en.disabled]);

    pinc.add_variant(Variant::new("Disabled", 0));
    pinc.add_variant(Variant::new("Enabled", 1));
}
