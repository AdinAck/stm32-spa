use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        register::Register,
        variant::Variant,
    },
};

pub fn generate() -> Register {
    Register::new(
        "csr",
        0x80,
        (0..16).map(|i| {
            Field::new(
                format!("sof{i}"),
                i,
                1,
                Access::read(Numericity::enumerated([
                    Variant::new("NoEvent", 0),
                    Variant::new("Occurred", 1),
                ])),
            )
            .docs(["Synchronization overrun event flag"])
        }),
    )
}
