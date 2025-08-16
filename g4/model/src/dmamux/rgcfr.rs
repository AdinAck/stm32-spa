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
        "rgcfr",
        0x144,
        (0..4).map(|i| {
            Field::new(
                format!("cof{i}"),
                i,
                1,
                Access::write(Numericity::enumerated([
                    Variant::new("Noop", 0).inert(),
                    Variant::new("Clear", 1),
                ])),
            )
            .docs(["Clear trigger overrun event flag"])
        }),
    )
}
