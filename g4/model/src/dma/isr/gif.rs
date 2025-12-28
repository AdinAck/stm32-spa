use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn gif<'cx>(isr: &mut RegisterEntry<'cx>, i: u8) {
    let mut gif = isr.add_read_field(
        Field::new(format!("gif{}", i + 1), i * 4, 1).docs(["Global interrupt flag"]),
    );

    gif.add_variant(Variant::new("NoEvent", 0));
    gif.add_variant(Variant::new("Occurred", 1));
}
