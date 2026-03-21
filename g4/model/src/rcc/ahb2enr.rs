use peripherals::{prelude::*, rcc::enr::en};
use phm::model::PeripheralEntry;

pub struct Output {
    pub gpioaen: en::Output,
    pub gpioben: en::Output,
    pub gpiocen: en::Output,
    pub gpioden: en::Output,
    pub gpioeen: en::Output,
    pub gpiofen: en::Output,
    pub gpiogen: en::Output,

    pub adc12en: en::Output,
    pub adc345en: en::Output,

    pub dac1en: en::Output,
    pub dac2en: en::Output,
    pub dac3en: en::Output,
    pub dac4en: en::Output,

    pub aesen: en::Output,

    pub rngen: en::Output,
}

pub fn ahb2enr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut ahb2 = rcc.enr("ahb2", 0x4c, 0, None);

    Output {
        gpioaen: ahb2.en("gpioa", 0).en(),
        gpioben: ahb2.en("gpiob", 1).en(),
        gpiocen: ahb2.en("gpioc", 2).en(),
        gpioden: ahb2.en("gpiod", 3).en(),
        gpioeen: ahb2.en("gpioe", 4).en(),
        gpiofen: ahb2.en("gpiof", 5).en(),
        gpiogen: ahb2.en("gpiog", 6).en(),

        adc12en: ahb2.en("adc12", 13).en(),
        adc345en: ahb2.en("adc345", 14).en(),

        dac1en: ahb2.en("dac1", 16).en(),
        dac2en: ahb2.en("dac2", 17).en(),
        dac3en: ahb2.en("dac3", 18).en(),
        dac4en: ahb2.en("dac4", 19).en(),

        aesen: ahb2.en("aes", 24).en(),

        rngen: ahb2.en("rng", 26).en(),
    }
}
