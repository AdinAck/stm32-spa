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
        "cfr",
        0x84,
        (0..16).map(|i| {
            Field::new(
                format!("csof{i}"),
                i,
                1,
                Access::write(Numericity::enumerated([
                    Variant::new("Noop", 0).inert(),
                    Variant::new("Clear", 1),
                ])),
            )
            .docs(["Clear synchronization overrun event flag"])
        }),
    )
}
