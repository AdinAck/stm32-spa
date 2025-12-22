use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn spol<'cx>(ccr: &mut RegisterEntry<'cx>) {
    let mut spol =
        ccr.add_store_field(Field::new("spol", 17, 2).docs(["Synchronization polarity"]));

    spol.add_variant(Variant::new("NoEvent", 0));
    spol.add_variant(Variant::new("RisingEdge", 1));
    spol.add_variant(Variant::new("FallingEdge", 2));
    spol.add_variant(Variant::new("RisingAndFallingEdge", 3));
}
