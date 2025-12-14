use proto_hal_model::{Entitlement, Field, Register, model::PeripheralEntry};

use crate::rcc::enr::add_field;

pub struct Output {
    pub gpioaen: Entitlement,
    pub gpioben: Entitlement,
    pub gpiocen: Entitlement,
    pub gpioden: Entitlement,
    pub gpioeen: Entitlement,
    pub gpiofen: Entitlement,
    pub gpiogen: Entitlement,
    pub adc12en: Entitlement,
    pub adc345en: Entitlement,
    pub dac1en: Entitlement,
    pub dac2en: Entitlement,
    pub dac3en: Entitlement,
    pub dac4en: Entitlement,
    pub aesen: Entitlement,
    pub rngen: Entitlement,
}

pub fn ahb2enr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut ahb2enr = rcc.add_register(Register::new("ahb2enr", 0x4c).reset(0));

    Output {
        gpioaen: add_field(&mut ahb2enr, Field::new("gpioaen", 0, 1)),
        gpioben: add_field(&mut ahb2enr, Field::new("gpioben", 1, 1)),
        gpiocen: add_field(&mut ahb2enr, Field::new("gpiocen", 2, 1)),
        gpioden: add_field(&mut ahb2enr, Field::new("gpioden", 3, 1)),
        gpioeen: add_field(&mut ahb2enr, Field::new("gpioeen", 4, 1)),
        gpiofen: add_field(&mut ahb2enr, Field::new("gpiofen", 5, 1)),
        gpiogen: add_field(&mut ahb2enr, Field::new("gpiogen", 6, 1)),
        adc12en: add_field(&mut ahb2enr, Field::new("adc12en", 13, 1)),
        adc345en: add_field(&mut ahb2enr, Field::new("adc345en", 14, 1)),
        dac1en: add_field(&mut ahb2enr, Field::new("dac1en", 16, 1)),
        dac2en: add_field(&mut ahb2enr, Field::new("dac2en", 17, 1)),
        dac3en: add_field(&mut ahb2enr, Field::new("dac3en", 18, 1)),
        dac4en: add_field(&mut ahb2enr, Field::new("dac4en", 19, 1)),
        aesen: add_field(&mut ahb2enr, Field::new("aesen", 24, 1)),
        rngen: add_field(&mut ahb2enr, Field::new("rngen", 26, 1)),
    }
}
