use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn cteif<'cx>(ifcr: &mut RegisterEntry<'cx>, i: u8) {
    let mut cteif = ifcr.add_write_field(
        Field::new(format!("cteif{}", i + 1), i * 4 + 3, 1).docs(["Transfer error flag clear"]),
    );

    cteif.add_variant(Variant::new("Noop", 0).inert());
    cteif.add_variant(Variant::new("Clear", 1));
}
