use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

use crate::gpio::Instance;

pub fn generate(i: u8, instance: Instance) -> Field {
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
    .reset(match (instance, i) {
        (Instance::A, 13) => "VeryHigh",
        _ => "Low",
    })
}
