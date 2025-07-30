use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        variant::Variant,
    },
};

pub fn generate() -> Field {
    Field::new(
        "vrs",
        4,
        2,
        Access::read_write(Numericity::enumerated([
            Variant::new("R2V048", 0).docs(["2.048v."]),
            Variant::new("R2V500", 1).docs(["2.500v"]),
            Variant::new("R2V900", 2).docs(["2.900v"]),
        ])),
    )
    .reset("R2V048")
}
