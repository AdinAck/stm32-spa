use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn oie<'cx>(rgcr: &mut RegisterEntry<'cx>) {
    let mut oie =
        rgcr.add_store_field(Field::new("oie", 8, 1).docs(["Trigger overrun interrupt enable"]));

    oie.add_variant(Variant::new("Disable", 0));
    oie.add_variant(Variant::new("Enable", 1));
}
