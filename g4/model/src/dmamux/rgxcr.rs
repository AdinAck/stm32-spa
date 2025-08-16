use proto_hal_build::ir::{
    access::{Access, AccessProperties, HardwareAccess, ReadWrite},
    structures::{
        entitlement::Entitlement,
        field::{Field, Numericity},
        register::Register,
        variant::Variant,
    },
};

pub fn generate(channel: u8) -> Register {
    Register::new(
        format!("rg{}cr", channel),
        0x100 + channel as u32 * 4,
        [
            Field::new("sig_id", 0, 5, Access::read_write(Numericity::Numeric))
                .docs(["Signal identification"]),
            Field::new(
                "oie",
                8,
                1,
                Access::read_write(Numericity::enumerated([
                    Variant::new("Disabled", 0),
                    Variant::new("Enabled", 1),
                ])),
            )
            .docs(["Trigger overrun interrupt enable"]),
            Field::new(
                "ge",
                16,
                1,
                Access::read_write(Numericity::enumerated([
                    Variant::new("Disabled", 0),
                    Variant::new("Enabled", 1),
                ])),
            )
            .docs(["DMA request generator channel enable"]),
            Field::new(
                "gpol",
                17,
                2,
                Access::read_write(Numericity::enumerated([
                    Variant::new("NoEvent", 0),
                    Variant::new("RisingEdge", 1),
                    Variant::new("FallingEdge", 2),
                    Variant::new("RisingAndFallingEdges", 3),
                ])),
            )
            .docs(["DMA request generator trigger polarity"]),
            Field::new(
                "gnbreq",
                19,
                5,
                Access::ReadWrite(ReadWrite::Asymmetrical {
                    read: AccessProperties::numeric(),
                    write: AccessProperties::numeric().entitlements([Entitlement::to(format!(
                        "dmamux::rg{channel}cr::ge::Disabled"
                    ))]),
                }),
            )
            .hardware_access(HardwareAccess::ReadOnly)
            .docs(["Number of DMA requests to be generated (minus 1)"]),
        ]
        .into_iter()
        .map(|field| field.reset(0)), // reset value of rgcxr register is 0
    )
}
