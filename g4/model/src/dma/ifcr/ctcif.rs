use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn ctcif<'cx>(ifcr: &mut RegisterEntry<'cx>, i: u8) {
    let mut ctcif = ifcr.add_write_field(
        Field::new(format!("ctcif{}", i + 1), i * 4 + 1, 1).docs(["Transfer complete flag clear"]),
    );

    ctcif.add_variant(Variant::new("Noop", 0).inert());
    ctcif.add_variant(Variant::new("Clear", 1));
}
