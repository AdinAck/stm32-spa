use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate(i: u8) -> Field {
    Field::new(
        format!("ospeed{i}"),
        i * 2,
        2,
        Access::read_write(Numericity::enumerated([
            Variant::new("Low", 0),
            Variant::new("Medium", 1),
            Variant::new("High", 2),
            Variant::new("VeryHigh", 3),
        ])),
    )
}
