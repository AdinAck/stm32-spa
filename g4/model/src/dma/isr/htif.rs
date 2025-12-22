use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn htif<'cx>(isr: &mut RegisterEntry<'cx>, i: u8) {
    let mut htif = isr.add_read_field(
        Field::new(format!("htif{}", i + 1), i * 4 + 2, 1).docs(["Half transfer flag"]),
    );

    htif.add_variant(Variant::new("NoEvent", 0));
    htif.add_variant(Variant::new("Occurred", 1));
}
