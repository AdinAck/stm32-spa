use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate(x: u8, offset: u8) -> Field {
    Field::new(
        format!("pif{x}"),
        offset,
        1,
        Access::read_write_asymmetrical(
            Numericity::enumerated([Variant::new("Idle", 0), Variant::new("Pending", 1)]),
            Numericity::enumerated([Variant::new("Retain", 0), Variant::new("Clear", 1)]),
        ),
    )
}
