use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn tcif<'cx>(isr: &mut RegisterEntry<'cx>, i: u8) {
    let mut tcif = isr.add_read_field(
        Field::new(format!("tcif{}", i + 1), i * 4 + 1, 1).docs(["Transfer complete flag"]),
    );

    tcif.add_variant(Variant::new("NoEvent", 0));
    tcif.add_variant(Variant::new("Occurred", 1));
}
