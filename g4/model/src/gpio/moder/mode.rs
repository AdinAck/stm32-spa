use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate(i: u8) -> Field {
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
}
