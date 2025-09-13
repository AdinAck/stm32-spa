use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate() -> Field {
    Field::new(
        "envr",
        0,
        1,
        Access::read_write(Numericity::enumerated([
            Variant::new("Disabled", 0),
            Variant::new("Enabled", 1),
        ])),
    )
}
