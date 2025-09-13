use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        register::Register,
    },
};

pub fn generate() -> Register {
    Register::new(
        "pol",
        0x14,
        [Field::new(
            "pol",
            0,
            32,
            Access::read_write(Numericity::Numeric),
        )],
    )
    .reset(0x04c1_1db7)
}
