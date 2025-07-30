use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate() -> Field {
    Field::new(
        "vrr",
        3,
        1,
        Access::read(Numericity::enumerated([
            Variant::new("NotReady", 0),
            Variant::new("Ready", 1),
        ])),
    )
}
