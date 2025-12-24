use proto_hal_model::{Entitlement, Field, Register, model::PeripheralEntry};

use crate::rcc::enr::add_field;

pub struct Output {
    pub syscfgen: Entitlement,
    pub tim1en: Entitlement,
    pub spi1en: Entitlement,
    pub tim8en: Entitlement,
    pub usart1en: Entitlement,
    pub spi4en: Entitlement,
    pub tim15en: Entitlement,
    pub tim16en: Entitlement,
    pub tim17en: Entitlement,
    pub tim20en: Entitlement,
    pub sai1en: Entitlement,
    pub hrtim1en: Entitlement,
}

pub fn apb2enr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut apb2enr = rcc.add_register(Register::new("apb2enr", 0x60).reset(0));

    Output {
        syscfgen: add_field(&mut apb2enr, Field::new("syscfgen", 0, 1).docs(["This field enables/disables the peripherals clocks for SYSCFG, COMP, VREFBUF, and OPAMP."])),
        tim1en: add_field(&mut apb2enr, Field::new("tim1en", 11, 1)),
        spi1en: add_field(&mut apb2enr, Field::new("spi1en", 12, 1)),
        tim8en: add_field(&mut apb2enr, Field::new("tim8en", 13, 1)),
        usart1en: add_field(&mut apb2enr, Field::new("usart1en", 14, 1)),
        spi4en: add_field(&mut apb2enr, Field::new("spi4en", 15, 1)),
        tim15en: add_field(&mut apb2enr, Field::new("tim15en", 16, 1)),
        tim16en: add_field(&mut apb2enr, Field::new("tim16en", 17, 1)),
        tim17en: add_field(&mut apb2enr, Field::new("tim17en", 18, 1)),
        tim20en: add_field(&mut apb2enr, Field::new("tim20en", 20, 1)),
        sai1en: add_field(&mut apb2enr, Field::new("sai1en", 21, 1)),
        hrtim1en: add_field(&mut apb2enr, Field::new("hrtim1en", 26, 1)),
    }
}
