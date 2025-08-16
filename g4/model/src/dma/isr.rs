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
        "isr",
        0x00,
        (0..8).flat_map(|i| {
            let channel = i + 1;
            [
                Field::new(
                    format!("gif{channel}"),
                    i * 4,
                    1,
                    Access::read(Numericity::enumerated([
                        Variant::new("NoEvent", 0),
                        Variant::new("Occurred", 1),
                    ])),
                )
                .docs(["Global interrupt flag"]),
                Field::new(
                    format!("tcif{channel}"),
                    i * 4 + 1,
                    1,
                    Access::read(Numericity::enumerated([
                        Variant::new("NoEvent", 0),
                        Variant::new("Occurred", 1),
                    ])),
                )
                .docs(["Transfer complete flag"]),
                Field::new(
                    format!("htif{channel}"),
                    i * 4 + 2,
                    1,
                    Access::read(Numericity::enumerated([
                        Variant::new("NoEvent", 0),
                        Variant::new("Occurred", 1),
                    ])),
                )
                .docs(["Half transfer flag"]),
                Field::new(
                    format!("teif{channel}"),
                    i * 4 + 3,
                    1,
                    Access::read(Numericity::enumerated([
                        Variant::new("NoEvent", 0),
                        Variant::new("Occurred", 1),
                    ])),
                )
                .docs(["Transfer error flag"]),
            ]
        }),
    )
}
