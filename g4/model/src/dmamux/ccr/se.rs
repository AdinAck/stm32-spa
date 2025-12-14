use proto_hal_model::{Entitlement, Field, Variant, model::RegisterEntry};

pub struct Output {
    pub disabled: Entitlement,
}

pub fn se<'cx>(ccr: &mut RegisterEntry<'cx>) -> Output {
    let mut se = ccr.add_store_field(Field::new("se", 16, 1).docs(["Synchronization enable"]));

    let disabled = se
        .add_variant(Variant::new("Disabled", 0))
        .make_entitlement();
    se.add_variant(Variant::new("Enabled", 1));

    Output { disabled }
}
