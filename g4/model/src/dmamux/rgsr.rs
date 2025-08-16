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
        "rgsr",
        0x140,
        (0..4).map(|i| {
            Field::new(
                format!("of{i}"),
                i,
                1,
                Access::read(Numericity::enumerated([
                    Variant::new("NoEvent", 0),
                    Variant::new("Occurred", 1),
                ])),
            )
            .docs(["Trigger overrun event flag"])
        }),
    )
}
