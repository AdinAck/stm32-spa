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
        "ifcr",
        0x04,
        (0..8).flat_map(|i| {
            let channel = i + 1;

            [
                Field::new(
                    format!("cgif{channel}"),
                    i * 4,
                    1,
                    Access::write(Numericity::enumerated([
                        Variant::new("Noop", 0).inert(),
                        Variant::new("Clear", 1),
                    ])),
                )
                .docs(["Global interrupt flag clear"]),
                Field::new(
                    format!("ctcif{channel}"),
                    i * 4 + 1,
                    1,
                    Access::write(Numericity::enumerated([
                        Variant::new("Noop", 0).inert(),
                        Variant::new("Clear", 1),
                    ])),
                )
                .docs(["Transfer complete flag clear"]),
                Field::new(
                    format!("chtif{channel}"),
                    i * 4 + 2,
                    1,
                    Access::write(Numericity::enumerated([
                        Variant::new("Noop", 0).inert(),
                        Variant::new("Clear", 1),
                    ])),
                )
                .docs(["Half transfer flag clear"]),
                Field::new(
                    format!("cteif{channel}"),
                    i * 4 + 3,
                    1,
                    Access::write(Numericity::enumerated([
                        Variant::new("Noop", 0).inert(),
                        Variant::new("Clear", 1),
                    ])),
                )
                .docs(["Transfer error flag clear"]),
            ]
        }),
    )
}
