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
        format!("mode{i}"),
        i * 2,
        2,
        Access::read_write(Numericity::enumerated([
            Variant::new("Input", 0),
            Variant::new("Output", 1),
            Variant::new("Alternate", 2),
            Variant::new("Analog", 3),
        ])),
    )
    .reset(match (instance, i) {
        (Instance::A, 13..=15) | (Instance::B, 3 | 4) => "Alternate",
        _ => "Input",
    })
}
