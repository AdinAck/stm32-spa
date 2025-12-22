use proto_hal_model::{Field, model::RegisterEntry};

use crate::dma::ccr::en;

pub fn ndt<'cx>(cndtr: &mut RegisterEntry<'cx>, en: en::Output) {
    let mut ndt = cndtr
        .add_volatile_store_field(Field::new("ndt", 0, 16).docs(["Number of data to transfer"]));

    ndt.hardware_write_entitlements([en.enabled]);
    ndt.write_entitlements([en.disabled]);
}
