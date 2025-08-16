use proto_hal_build::ir::{
    access::{Access, AccessProperties, ReadWrite},
    structures::{entitlement::Entitlement, field::Field, register::Register},
};

fn offset(channel: u8) -> u32 {
    0x10 + 0x14 * channel as u32
}

pub fn generate(instance: super::Instance, channel: u8) -> Register {
    Register::new(
        format!("cpar{channel}"),
        offset(channel),
        [
            (
                "pa8",
                0,
                32,
                [Entitlement::to(format!(
                    "{}::ccr{}::psize::Bits8",
                    instance.ident(),
                    channel,
                ))],
                ["Peripheral address for 8 bit transfers."],
            ),
            (
                "pa16",
                1,
                31,
                [Entitlement::to(format!(
                    "{}::ccr{}::psize::Bits16",
                    instance.ident(),
                    channel,
                ))],
                ["Peripheral address for 16 bit transfers."],
            ),
            (
                "pa32",
                2,
                30,
                [Entitlement::to(format!(
                    "{}::ccr{}::psize::Bits32",
                    instance.ident(),
                    channel,
                ))],
                ["Peripheral address for 32 bit transfers."],
            ),
        ]
        .map(|(ident, offset, width, entitlements, docs)| {
            Field::new(
                ident,
                offset,
                width,
                Access::ReadWrite(ReadWrite::Asymmetrical {
                    read: AccessProperties::numeric(),
                    write: AccessProperties::numeric().entitlements([Entitlement::to(format!(
                        "{}::ccr{}::en::Disabled",
                        instance.ident(),
                        channel,
                    ))]),
                }),
            )
            .reset(0)
            .entitlements(entitlements)
            .docs(docs)
        }),
    )
}
