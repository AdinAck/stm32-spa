use peripherals::{prelude::*, rcc::enr};
use proto_hal_model::model::PeripheralEntry;

pub struct Output {
    pub gpioaen: enr::Output,
    pub gpioben: enr::Output,
    pub gpiocen: enr::Output,
    pub gpioden: enr::Output,
    pub gpioeen: enr::Output,
    pub gpiofen: enr::Output,
    pub gpiogen: enr::Output,

    pub adc12en: enr::Output,
    pub adc345en: enr::Output,

    pub dac1en: enr::Output,
    pub dac2en: enr::Output,
    pub dac3en: enr::Output,
    pub dac4en: enr::Output,

    pub aesen: enr::Output,

    pub rngen: enr::Output,
}

pub fn ahb2enr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut ahb2 = rcc.enr("ahb2", 0x4c, 0, None);

    Output {
        gpioaen: ahb2.en("gpioa", 0),
        gpioben: ahb2.en("gpiob", 1),
        gpiocen: ahb2.en("gpioc", 2),
        gpioden: ahb2.en("gpiod", 3),
        gpioeen: ahb2.en("gpioe", 4),
        gpiofen: ahb2.en("gpiof", 5),
        gpiogen: ahb2.en("gpiog", 6),

        adc12en: ahb2.en("adc12", 13),
        adc345en: ahb2.en("adc345", 14),

        dac1en: ahb2.en("dac1", 16),
        dac2en: ahb2.en("dac2", 17),
        dac3en: ahb2.en("dac3", 18),
        dac4en: ahb2.en("dac4", 19),

        aesen: ahb2.en("aes", 24),

        rngen: ahb2.en("rng", 26),
    }
}
