use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate() -> Field {
    Field::new(
        "hiz",
        1,
        1,
        Access::read_write(Numericity::enumerated([
            Variant::new("Connected", 0)
                .docs(["The Vref+ pin is connected to the internal reference."]),
            Variant::new("Disconnected", 1)
                .docs(["The Vref+ pin is *not* connected to the internal reference."]),
        ])),
    )
    .reset("Disconnected")
}
