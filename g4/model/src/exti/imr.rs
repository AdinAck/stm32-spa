pub mod im;

use proto_hal_build::ir::structures::register::Register;

#[derive(Clone, Copy)]
pub enum Instance {
    I1,
}

impl Instance {
    fn ident(&self) -> String {
        match self {
            Instance::I1 => "imr1",
        }
        .to_string()
    }

    fn offset(&self) -> u32 {
        match self {
            Instance::I1 => 0x00,
        }
    }
}

pub fn generate(instance: Instance) -> Register {
    Register::new(
        instance.ident(),
        instance.offset(),
        // TODO
        match instance {
            Instance::I1 => (0..16).map(|x| im::generate(x, x)),
        },
    )
    .reset(0)
    .docs(["Interrupt Mask Register"])
}
