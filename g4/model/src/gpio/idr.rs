use proto_hal_model::{Register, model::PeripheralEntry};

use crate::gpio::idr::id::id;

pub mod id;

pub fn idr<'cx>(gpio: &mut PeripheralEntry<'cx>) {
    let mut idr = gpio.add_register(Register::new("idr", 0x10));

    for i in 0..16 {
        id(&mut idr, i);
    }
}
