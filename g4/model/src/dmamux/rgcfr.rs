use proto_hal_model::{Field, Register, Variant, model::PeripheralEntry};

pub fn rgcfr<'cx>(dmamux: &mut PeripheralEntry<'cx>) {
    let mut rgcfr = dmamux.add_register(Register::new("rgcfr", 0x144));

    for i in 0..4 {
        let mut of = rgcfr.add_write_field(
            Field::new(format!("cof{i}"), i, 1).docs(["Clear trigger overrun event flag"]),
        );

        of.add_variant(Variant::new("Noop", 0).inert());
        of.add_variant(Variant::new("Clear", 1));
    }
}
