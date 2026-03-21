use peripherals::{prelude::*, rcc::enr::en};
use phm::model::PeripheralEntry;

pub struct Output {
    pub gpioaen: en::Output,
    pub gpioben: en::Output,
    pub gpiocen: en::Output,
    pub gpioden: en::Output,

    pub gpiofen: en::Output,
}

pub fn iopenr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut iop = rcc.enr("iop", 0x34, 0, None);

    Output {
        gpioaen: iop.en("gpioa", 0).en(),
        gpioben: iop.en("gpiob", 1).en(),
        gpiocen: iop.en("gpioc", 2).en(),
        gpioden: iop.en("gpiod", 3).en(),

        gpiofen: iop.en("gpiof", 5).en(),
    }
}
