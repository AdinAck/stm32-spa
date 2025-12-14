use proto_hal_model::{Register, model::PeripheralEntry};

use crate::gpio::otyper::ot::ot;

pub mod ot;

pub fn otyper<'cx>(gpio: &mut PeripheralEntry<'cx>) {
    let mut otyper = gpio.add_register(Register::new("otyper", 4).reset(0));

    for i in 0..16 {
        ot(&mut otyper, i);
    }
}
