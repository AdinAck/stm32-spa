pub mod dmareq;

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
        format!("c{channel}cr"),
        channel as u32 * 4,
        [
            dmareq::generate(),
            Field::new(
                "soie",
                8,
                1,
                Access::read_write(Numericity::enumerated([
                    Variant::new("Disabled", 0),
                    Variant::new("Enabled", 1),
                ])),
            )
            .docs(["Synchronization overrun interrupt enable"]),
            Field::new(
                "ege",
                9,
                1,
                Access::read_write(Numericity::enumerated([
                    Variant::new("Disabled", 0),
                    Variant::new("Enabled", 1),
                ])),
            )
            .docs(["Event generation enable"]),
            Field::new(
                "se",
                16,
                1,
                Access::read_write(Numericity::enumerated([
                    Variant::new("Disabled", 0),
                    Variant::new("Enabled", 1),
                ])),
            )
            .docs(["Synchronization enable"]),
            Field::new(
                "spol",
                17,
                2,
                Access::read_write(Numericity::enumerated([
                    Variant::new("NoEvent", 0),
                    Variant::new("RisingEdge", 1),
                    Variant::new("FallingEdge", 2),
                    Variant::new("RisingAndFallingEdges", 3),
                ])),
            )
            .docs(["Synchronization polarity"]),
            Field::new(
                "nbreq",
                19,
                5,
                Access::ReadWrite(ReadWrite::Asymmetrical {
                    read: AccessProperties::numeric(),
                    write: AccessProperties::numeric().entitlements([
                        Entitlement::to(format!("dmamux::c{}cr::se::Disabled", channel)),
                        Entitlement::to(format!("dmamux::c{}cr::ege::Disabled", channel)),
                    ]),
                }),
            )
            .hardware_access(HardwareAccess::ReadOnly)
            .docs(["Number of DMA requests minus 1 to forward"]),
            Field::new("sync_id", 24, 5, Access::read_write(Numericity::Numeric))
                .docs(["Synchronization identification"]),
        ]
        .into_iter()
        .map(|field| {
            field.reset(0) // reset value of cxcr register is 0
        }),
    )
}
