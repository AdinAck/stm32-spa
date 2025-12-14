use proto_hal_model::{Field, Register, Variant, model::PeripheralEntry};

pub fn csr<'cx>(dmamux: &mut PeripheralEntry<'cx>) {
    let mut csr = dmamux.add_register(Register::new("csr", 0x80));

    for i in 0..16 {
        let mut sof = csr.add_read_field(
            Field::new(format!("sof{i}"), i, 1).docs(["Synchronization overrun event flag"]),
        );

        sof.add_variant(Variant::new("NoEvent", 0));
        sof.add_variant(Variant::new("Occurred", 1));
    }
}
