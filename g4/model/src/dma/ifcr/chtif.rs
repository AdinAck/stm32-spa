use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn chtif<'cx>(ifcr: &mut RegisterEntry<'cx>, i: u8) {
    let mut chtif = ifcr.add_write_field(
        Field::new(format!("chtif{}", i + 1), i * 4 + 2, 1).docs(["Half transfer flag clear"]),
    );

    chtif.add_variant(Variant::new("Noop", 0).inert());
    chtif.add_variant(Variant::new("Clear", 1));
}
