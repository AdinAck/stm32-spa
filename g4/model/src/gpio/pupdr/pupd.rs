use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate(i: u8) -> Field {
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
}
