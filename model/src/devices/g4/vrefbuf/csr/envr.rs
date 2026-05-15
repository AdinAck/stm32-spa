use phm::{Entitlement, Field, Variant, model::RegisterEntry};

pub struct Output {
    pub disabled: Entitlement,
}

pub fn envr<'cx>(csr: &mut RegisterEntry<'cx>) -> Output {
    let mut envr = csr.add_store_field(Field::new("envr", 0, 1));

    envr.add_variant(Variant::new("Enabled", 1));

    Output {
        disabled: envr
            .add_variant(Variant::new("Disabled", 0))
            .make_entitlement(),
    }
}
