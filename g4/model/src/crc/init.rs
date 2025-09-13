use proto_hal_build::ir::{
    access::Access,
    structures::{
        field::{Field, Numericity},
        register::Register,
    },
};

pub fn generate() -> Register {
    Register::new(
        "init",
        0x10,
        [Field::new(
            "init",
            0,
            32,
            Access::read_write(Numericity::Numeric),
        )],
    )
    .reset(u32::MAX)
}
