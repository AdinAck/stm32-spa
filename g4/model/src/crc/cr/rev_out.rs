use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate() -> Field {
    Field::new(
        "rev_in",
        7,
        1,
        Access::read_write(Numericity::enumerated([
            Variant::new("NoEffect", 0),
            Variant::new("Reversed", 1),
        ])),
    )
}
