use proto_hal_model::{Field, Variant, model::RegisterEntry};

use crate::dma::ccr::en;

pub fn pl<'cx>(ccr: &mut RegisterEntry<'cx>, en: en::Output) {
    let mut pl = ccr.add_store_field(Field::new("pl", 12, 2).docs(["Priority level"]));

    pl.write_entitlements([en.disabled]);

    pl.add_variant(Variant::new("Low", 0));
    pl.add_variant(Variant::new("Medium", 1));
    pl.add_variant(Variant::new("High", 2));
    pl.add_variant(Variant::new("VeryHigh", 3));
}
