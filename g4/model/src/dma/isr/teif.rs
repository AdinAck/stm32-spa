use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn teif<'cx>(isr: &mut RegisterEntry<'cx>, i: u8) {
    let mut teif = isr.add_read_field(
        Field::new(format!("teif{}", i + 1), i * 4 + 3, 1).docs(["Transfer error flag"]),
    );

    teif.add_variant(Variant::new("NoEvent", 0));
    teif.add_variant(Variant::new("Occurred", 1));
}
