pub mod ft;

use proto_hal_build::ir::structures::register::Register;

#[derive(Clone, Copy)]
pub enum Instance {
    I1,
}

impl Instance {
    fn ident(&self) -> String {
        match self {
            Instance::I1 => "ftsr1",
        }
        .to_string()
    }

    fn offset(&self) -> u32 {
        match self {
            Instance::I1 => 0x0c,
        }
    }
}

pub fn generate(instance: Instance) -> Register {
    Register::new(
        instance.ident(),
        instance.offset(),
        // TODO
        match instance {
            Instance::I1 => (0..16).map(|x| ft::generate(x, x)),
        },
    )
    .docs(["Falling Trigger Selection Register"])
}
