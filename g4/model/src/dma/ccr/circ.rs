use proto_hal_model::{Entitlement, Field, Variant, model::RegisterEntry};

use crate::dma::ccr::en;

pub struct Output {
    pub disabled: Entitlement,
}

pub fn circ<'cx>(ccr: &mut RegisterEntry<'cx>, en: en::Output) -> Output {
    let mut circ = ccr.add_store_field(Field::new("circ", 5, 1).docs(["Circular mode"]));

    circ.write_entitlements([en.disabled]);

    let disabled = circ
        .add_variant(Variant::new("Disabled", 0))
        .make_entitlement();
    circ.add_variant(Variant::new("Enabled", 1));

    Output { disabled }
}
