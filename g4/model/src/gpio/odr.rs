use proto_hal_model::{Register, model::PeripheralEntry};

use crate::gpio::odr::od::od;

pub mod od;

pub fn odr<'cx>(gpio: &mut PeripheralEntry<'cx>) {
    let mut odr = gpio.add_register(Register::new("odr", 0x14).reset(0));

    for i in 0..16 {
        od(&mut odr, i);
    }
}
