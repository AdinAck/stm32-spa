use proto_hal_model::{Entitlement, Field, Variant, model::RegisterEntry};

#[derive(Debug, Clone, Copy)]
pub struct Output {
    pub disabled: Entitlement,
    pub enabled: Entitlement,
}

pub fn en<'cx>(ccr: &mut RegisterEntry<'cx>) -> Output {
    let mut en = ccr.add_volatile_store_field(Field::new("en", 0, 1).docs(["Channel enable"]));

    let disabled = en
        .add_variant(Variant::new("Disabled", 0))
        .make_entitlement();
    let enabled = en
        .add_variant(Variant::new("Enabled", 1))
        .make_entitlement();

    en.hardware_write_entitlements([enabled]);

    Output { disabled, enabled }
}
