use proto_hal_build::ir::{
    access::{Access, HardwareAccess, ReadWrite},
    structures::{
        entitlement::Entitlement,
        field::{Field, Numericity},
        register::Register,
        variant::Variant,
    },
};

fn offset(channel: u8) -> u32 {
    0x08 + 0x14 * channel as u32
}

pub fn generate(instance: super::Instance, channel: u8) -> Register {
    Register::new(
        format!("ccr{channel}"),
        offset(channel),
        [Field::new(
            "en",
            0,
            1,
            Access::read_write(Numericity::enumerated([
                Variant::new("Disabled", 0),
                Variant::new("Enabled", 1),
            ])),
        )
        .docs(["Channel enable"])]
        .into_iter()
        .chain(
            [
                Field::new(
                    "tcie",
                    1,
                    1,
                    Access::read_write(Numericity::enumerated([
                        Variant::new("Disabled", 0),
                        Variant::new("Enabled", 1),
                    ])),
                )
                .docs(["Transfer complete interrupt enable"]),
                Field::new(
                    "htie",
                    2,
                    1,
                    Access::read_write(Numericity::enumerated([
                        Variant::new("Disabled", 0),
                        Variant::new("Enabled", 1),
                    ])),
                )
                .docs(["Half transfer interrupt enable"]),
                Field::new(
                    "teie",
                    3,
                    1,
                    Access::read_write(Numericity::enumerated([
                        Variant::new("Disabled", 0),
                        Variant::new("Enabled", 1),
                    ])),
                )
                .docs(["Transfer error interrupt enable"]),
                Field::new(
                    "dir",
                    4,
                    1,
                    Access::read_write(Numericity::enumerated([
                        Variant::new("ReadFromPeripheral", 0),
                        Variant::new("ReadFromMemory", 1),
                    ])),
                )
                .docs(["Data transfer direction"]),
                Field::new(
                    "circ",
                    5,
                    1,
                    Access::read_write(Numericity::enumerated([
                        Variant::new("Disabled", 0),
                        Variant::new("Enabled", 1),
                    ])),
                )
                .docs(["Circular mode"]),
                Field::new(
                    "pinc",
                    6,
                    1,
                    Access::read_write(Numericity::enumerated([
                        Variant::new("Disabled", 0),
                        Variant::new("Enabled", 1),
                    ])),
                )
                .docs(["Peripheral increment mode"]),
                Field::new(
                    "minc",
                    7,
                    1,
                    Access::read_write(Numericity::enumerated([
                        Variant::new("Disabled", 0),
                        Variant::new("Enabled", 1),
                    ])),
                )
                .docs(["Memory increment mode"]),
                Field::new(
                    "psize",
                    8,
                    2,
                    Access::read_write(Numericity::enumerated([
                        Variant::new("Bits8", 0),
                        Variant::new("Bits16", 1),
                        Variant::new("Bits32", 2),
                    ])),
                )
                .docs(["Peripheral size"]),
                Field::new(
                    "msize",
                    10,
                    2,
                    Access::read_write(Numericity::enumerated([
                        Variant::new("Bits8", 0),
                        Variant::new("Bits16", 1),
                        Variant::new("Bits32", 2),
                    ])),
                )
                .docs(["Memory size"]),
                Field::new(
                    "pl",
                    12,
                    2,
                    Access::read_write(Numericity::enumerated([
                        Variant::new("Low", 0),
                        Variant::new("Medium", 1),
                        Variant::new("High", 2),
                        Variant::new("VeryHigh", 3),
                    ])),
                )
                .docs(["Priority level"]),
                Field::new(
                    "mem2mem",
                    14,
                    1,
                    Access::read_write(Numericity::enumerated([
                        Variant::new("Disabled", 0),
                        Variant::new("Enabled", 1).entitlements([Entitlement::to(format!(
                            "{}::ccr{}::circ::Disabled",
                            instance.ident(),
                            channel,
                        ))]),
                    ])),
                )
                .docs(["Memory to memory mode"]),
            ]
            .map(|mut field| {
                field.access = if let Access::Write(write)
                | Access::ReadWrite(
                    ReadWrite::Symmetrical(write) | ReadWrite::Asymmetrical { write, .. },
                ) = field.access
                {
                    Access::ReadWrite(ReadWrite::Asymmetrical {
                        read: write.clone(),
                        write: write.entitlements([Entitlement::to(format!(
                            "{}::ccr{}::en::Disabled",
                            instance.ident(),
                            channel,
                        ))]),
                    })
                } else {
                    field.access
                };

                field.hardware_access(HardwareAccess::ReadOnly)
            }),
        )
        .map(|field| {
            field.reset(0) // reset value of ccr register is 0
        }),
    )
}
