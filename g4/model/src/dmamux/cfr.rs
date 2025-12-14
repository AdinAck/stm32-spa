use proto_hal_model::{Field, Register, Variant, model::PeripheralEntry};

pub fn cfr<'cx>(dmamux: &mut PeripheralEntry<'cx>) {
    let mut cfr = dmamux.add_register(Register::new("cfr", 0x84));

    for i in 0..16 {
        let mut sof = cfr.add_read_field(
            Field::new(format!("csof{i}"), i, 1).docs(["Clear synchronization overrun event flag"]),
        );

        sof.add_variant(Variant::new("Noop", 0).inert());
        sof.add_variant(Variant::new("Clear", 1));
    }
}
