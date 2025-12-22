use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn cgif<'cx>(ifcr: &mut RegisterEntry<'cx>, i: u8) {
    let mut cgif = ifcr.add_write_field(
        Field::new(format!("cgif{}", i + 1), i * 4, 1).docs(["Global interrupt flag clear"]),
    );

    cgif.add_variant(Variant::new("Noop", 0).inert());
    cgif.add_variant(Variant::new("Clear", 1));
}
