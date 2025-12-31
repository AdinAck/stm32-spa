use peripherals::{prelude::*, rcc::enr};
use proto_hal_model::model::PeripheralEntry;

pub struct Output {
    pub syscfgen: enr::Output,

    pub tim1en: enr::Output,
    pub spi1en: enr::Output,
    pub tim8en: enr::Output,
    pub usart1en: enr::Output,
    pub spi4en: enr::Output,
    pub tim15en: enr::Output,
    pub tim16en: enr::Output,
    pub tim17en: enr::Output,

    pub tim20en: enr::Output,
    pub sai1en: enr::Output,

    pub hrtim1en: enr::Output,
}

pub fn apb2enr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut apb2 = rcc.enr("apb2", 0x60, 0, None);

    Output {
        syscfgen: apb2.en("syscfg", 0),

        tim1en: apb2.en("tim1", 1),
        spi1en: apb2.en("spi1", 2),
        tim8en: apb2.en("tim8", 3),
        usart1en: apb2.en("usart1", 4),
        spi4en: apb2.en("spi4", 5),
        tim15en: apb2.en("tim15", 6),
        tim16en: apb2.en("tim16", 13),
        tim17en: apb2.en("tim17", 14),

        tim20en: apb2.en("tim20", 16),
        sai1en: apb2.en("sai1", 17),

        hrtim1en: apb2.en("hrtim1", 18),
    }
}
