use proto_hal_model::{Field, Variant, model::RegisterEntry};

use crate::dma::ccr::{circ, en};

pub struct Entitlements {
    pub en: en::Output,
    pub circ: circ::Output,
}

pub fn mem2mem<'cx>(ccr: &mut RegisterEntry<'cx>, entitlements: Entitlements) {
    let mut mem2mem =
        ccr.add_store_field(Field::new("mem2mem", 14, 1).docs(["Memory to memory mode"]));

    mem2mem.write_entitlements([entitlements.en.disabled]);

    mem2mem.add_variant(Variant::new("Disabled", 0));
    mem2mem
        .add_variant(Variant::new("Enabled", 1))
        .statewise_entitlements([entitlements.circ.disabled]);
}
