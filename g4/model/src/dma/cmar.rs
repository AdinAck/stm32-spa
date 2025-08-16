use proto_hal_build::ir::{
    access::{Access, AccessProperties, HardwareAccess, ReadWrite},
    structures::{entitlement::Entitlement, field::Field, register::Register},
};

fn offset(channel: u8) -> u32 {
    0x14 + 0x14 * channel as u32
}

pub fn generate(instance: super::Instance, channel: u8) -> Register {
    Register::new(
        format!("cmar{channel}"),
        offset(channel),
        [
            (
                "ma8",
                0,
                32,
                [Entitlement::to(format!(
                    "{}::ccr{}::msize::Bits8",
                    instance.ident(),
                    channel,
                ))],
                ["Memory address for 8 bit transfers."],
            ),
            (
                "ma16",
                1,
                31,
                [Entitlement::to(format!(
                    "{}::ccr{}::msize::Bits16",
                    instance.ident(),
                    channel,
                ))],
                ["Memory address for 16 bit transfers."],
            ),
            (
                "ma32",
                2,
                30,
                [Entitlement::to(format!(
                    "{}::ccr{}::msize::Bits32",
                    instance.ident(),
                    channel,
                ))],
                ["Memory address for 32 bit transfers."],
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
            .hardware_access(HardwareAccess::ReadOnly)
            .docs(docs)
        }),
    )
}
