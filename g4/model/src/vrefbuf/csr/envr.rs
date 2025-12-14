use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn envr<'cx>(csr: &mut RegisterEntry<'cx>) {
    let mut envr = csr.add_store_field(Field::new("envr", 0, 1));

    envr.add_variant(Variant::new("Disabled", 0));
    envr.add_variant(Variant::new("Enabled", 1));
}
