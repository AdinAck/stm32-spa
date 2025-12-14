pub mod mode;

use proto_hal_model::{Register, model::PeripheralEntry};

use crate::gpio::{Instance, moder::mode::mode};

pub fn moder<'cx>(gpio: &mut PeripheralEntry<'cx>, instance: Instance) {
    let mut moder = gpio.add_register(
        Register::new("moder", 0)
            .reset(match instance {
                Instance::A => 0xabff_ffff,
                Instance::B => 0xffff_febf,
                _ => 0xffff_ffff,
            })
            .docs([
                "*Note: It is recommended to set PB8 to a different mode than the \
                analog one to limit the consumption that would occur if the pin is \
                left unconnected.*",
            ]),
    );

    for i in 0..16 {
        mode(&mut moder, i);
    }
}
