use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate(x: u8) -> Field {
    Field::new(
        format!("exti{x}"),
        (x % 4) * 4,
        4,
        Access::read_write(Numericity::enumerated([
            Variant::new("PA", 0),
            Variant::new("PB", 1),
            Variant::new("PC", 2),
            Variant::new("PD", 3),
            Variant::new("PE", 4),
            Variant::new("PF", 5),
            Variant::new("PG", 6),
        ])),
    )
}
