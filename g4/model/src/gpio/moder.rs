pub mod mode;

use proto_hal_build::ir::structures::register::Register;

use crate::gpio::Instance;

pub fn generate(instance: Instance) -> Register {
    Register::new("moder", 0, (0..16).map(|x| mode::generate(x, instance))).docs([
        "*Note: It is recommended to set PB8 to a different mode than the analog one to \
        limit the consumption that would occur if the pin is left unconnected.*",
    ])
}
