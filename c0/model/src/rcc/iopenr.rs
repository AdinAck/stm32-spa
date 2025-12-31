use peripherals::{prelude::*, rcc::enr};
use proto_hal_model::model::PeripheralEntry;

pub struct Output {
    pub gpioaen: enr::Output,
    pub gpioben: enr::Output,
    pub gpiocen: enr::Output,
    pub gpioden: enr::Output,

    pub gpiofen: enr::Output,
}

pub fn iopenr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut iop = rcc.enr("iop", 0x34, 0, None);

    Output {
        gpioaen: iop.en("gpioa", 0),
        gpioben: iop.en("gpiob", 1),
        gpiocen: iop.en("gpioc", 2),
        gpioden: iop.en("gpiod", 3),

        gpiofen: iop.en("gpiof", 5),
    }
}
