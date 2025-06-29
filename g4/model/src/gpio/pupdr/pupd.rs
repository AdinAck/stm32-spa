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
        format!("pupd{i}"),
        i * 2,
        2,
        Access::read_write(Numericity::enumerated([
            Variant::new("Floating", 0),
            Variant::new("PullUp", 1),
            Variant::new("PullDown", 2),
        ])),
    )
    .reset(match (instance, i) {
        (Instance::A, 13 | 15) | (Instance::B, 4) => "PullUp",
        (Instance::A, 14) => "PullDown",
        _ => "Floating",
    })
}
