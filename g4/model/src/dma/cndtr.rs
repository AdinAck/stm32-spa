use proto_hal_build::ir::{
    access::{Access, AccessProperties, HardwareAccess, ReadWrite},
    structures::{entitlement::Entitlement, field::Field, register::Register},
};

fn offset(channel: u8) -> u32 {
    0x0C + 0x14 * channel as u32
}

pub fn generate(instance: super::Instance, channel: u8) -> Register {
    Register::new(
        format!("cndtr{channel}"),
        offset(channel),
        [Field::new(
            "ndt",
            0,
            16,
            Access::ReadWrite(ReadWrite::Asymmetrical {
                read: AccessProperties::numeric(),
                write: AccessProperties::numeric().entitlements([Entitlement::to(format!(
                    "{}::ccr{}::en::Disabled",
                    instance.ident(),
                    channel,
                ))]),
            }),
        )
        .hardware_access(HardwareAccess::Write)
        .docs(["Number of data to transfer"])],
    )
}
