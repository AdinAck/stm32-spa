use proto_hal_model::{Field, Register, Variant, model::PeripheralEntry};

pub fn rgsr<'cx>(dmamux: &mut PeripheralEntry<'cx>) {
    let mut rgsr = dmamux.add_register(Register::new("rgsr", 0x140));

    for i in 0..4 {
        let mut of = rgsr.add_read_field(
            Field::new(format!("of{i}"), i, 1).docs(["Trigger overrun event flag"]),
        );

        of.add_variant(Variant::new("NoEvent", 0));
        of.add_variant(Variant::new("Occurred", 1));
    }
}
