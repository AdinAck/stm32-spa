use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate(x: u8, offset: u8) -> Field {
    Field::new(
        format!("rt{x}"),
        offset,
        1,
        Access::read_write(Numericity::enumerated([
            Variant::new("Disabled", 0),
            Variant::new("Enabled", 1),
        ])),
    )
    .reset("Disabled")
}
