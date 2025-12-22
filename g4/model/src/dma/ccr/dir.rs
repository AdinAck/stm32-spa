use proto_hal_model::{Field, Variant, model::RegisterEntry};

use crate::dma::ccr::en;

pub fn dir<'cx>(ccr: &mut RegisterEntry<'cx>, en: en::Output) {
    let mut dir = ccr.add_store_field(Field::new("dir", 4, 1).docs(["Data transfer direction"]));

    dir.write_entitlements([en.disabled]);

    dir.add_variant(Variant::new("ReadFromPeripheral", 0));
    dir.add_variant(Variant::new("ReadFromMemory", 1));
}
