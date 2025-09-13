use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate(i: u8) -> Field {
    Field::new(
        format!("ot{i}"),
        i,
        1,
        Access::read_write(Numericity::enumerated([
            Variant::new("PushPull", 0),
            Variant::new("OpenDrain", 1),
        ])),
    )
}
