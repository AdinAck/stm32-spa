use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn hiz<'cx>(csr: &mut RegisterEntry<'cx>) {
    let mut hiz = csr.add_store_field(Field::new("hiz", 1, 1));

    hiz.add_variant(
        Variant::new("Connected", 0)
            .docs(["The Vref+ pin is connected to the internal reference."]),
    );
    hiz.add_variant(
        Variant::new("Disconnected", 1)
            .docs(["The Vref+ pin is *not* connected to the internal reference."]),
    );
}
