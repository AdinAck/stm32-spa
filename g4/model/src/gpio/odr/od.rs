use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate(i: u8) -> Field {
    Field::new(
        format!("od{i}"),
        i,
        1,
        Access::read_write(Numericity::enumerated([
            Variant::new("Low", 0),
            Variant::new("High", 1),
        ])),
    )
}
