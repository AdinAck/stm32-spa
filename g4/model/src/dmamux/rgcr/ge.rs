use proto_hal_model::{Entitlement, Field, Variant, model::RegisterEntry};

pub struct Output {
    pub disabled: Entitlement,
}

pub fn ge<'cx>(rgcr: &mut RegisterEntry<'cx>) -> Output {
    let mut ge = rgcr
        .add_store_field(Field::new("ge", 16, 1).docs(["DMA request generator channel enable"]));

    let disabled = ge
        .add_variant(Variant::new("Disable", 0))
        .make_entitlement();
    ge.add_variant(Variant::new("Enable", 1));

    Output { disabled }
}
