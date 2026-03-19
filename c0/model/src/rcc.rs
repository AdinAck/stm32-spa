pub mod ahbenr;
pub mod iopenr;

use phm::{ModelBuilder, Peripheral};

use crate::rcc::{ahbenr::ahbenr, iopenr::iopenr};

pub struct Output {
    pub iopenr: iopenr::Output,
    pub ahbenr: ahbenr::Output,
}

pub fn rcc(model: &mut ModelBuilder) -> Output {
    let mut rcc = model.add_peripheral(Peripheral::new("rcc", 0x4002_1000));

    Output {
        iopenr: iopenr(&mut rcc),
        ahbenr: ahbenr(&mut rcc),
    }
}
