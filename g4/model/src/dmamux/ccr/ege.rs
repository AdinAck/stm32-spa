use proto_hal_model::{Entitlement, Field, Variant, model::RegisterEntry};

pub struct Output {
    pub disabled: Entitlement,
}

pub fn ege<'cx>(ccr: &mut RegisterEntry<'cx>) -> Output {
    let mut ege = ccr.add_store_field(Field::new("ege", 9, 1).docs(["Event generation enable"]));

    let disabled = ege
        .add_variant(Variant::new("Disabled", 0))
        .make_entitlement();
    ege.add_variant(Variant::new("Enabled", 1));

    Output { disabled }
}
