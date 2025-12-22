pub mod gif;
pub mod htif;
pub mod tcif;
pub mod teif;

use proto_hal_model::{Register, model::PeripheralEntry};

use gif::gif;
use htif::htif;
use tcif::tcif;
use teif::teif;

pub fn isr<'cx>(dma: &mut PeripheralEntry<'cx>) {
    let mut isr = dma.add_register(Register::new("isr", 0x00));

    for i in 0..8 {
        gif(&mut isr, i);
        tcif(&mut isr, i);
        htif(&mut isr, i);
        teif(&mut isr, i);
    }
}
