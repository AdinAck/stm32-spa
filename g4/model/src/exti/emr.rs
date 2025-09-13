pub mod em;

use proto_hal_build::ir::structures::register::Register;

#[derive(Clone, Copy)]
pub enum Instance {
    I1,
}

impl Instance {
    fn ident(&self) -> String {
        match self {
            Instance::I1 => "emr1",
        }
        .to_string()
    }

    fn offset(&self) -> u32 {
        match self {
            Instance::I1 => 0x04,
        }
    }
}

pub fn generate(instance: Instance) -> Register {
    Register::new(
        instance.ident(),
        instance.offset(),
        // TODO
        match instance {
            Instance::I1 => (0..16).map(|x| em::generate(x, x)),
        },
    )
    .reset(0)
    .docs(["Event Mask Register"])
}
