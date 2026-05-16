use crate::peripherals::{prelude::*, rcc::enr::en};
use phm::model::PeripheralEntry;

pub struct Output {
    pub syscfgen: en::EnSchema,

    pub tim1en: en::EnSchema,
    pub spi1en: en::EnSchema,
    pub tim8en: en::EnSchema,
    pub usart1en: en::EnSchema,
    pub spi4en: en::EnSchema,
    pub tim15en: en::EnSchema,
    pub tim16en: en::EnSchema,
    pub tim17en: en::EnSchema,

    pub tim20en: en::EnSchema,
    pub sai1en: en::EnSchema,

    pub hrtim1en: en::EnSchema,
}

pub fn apb2enr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut apb2 = rcc.enr("apb2", 0x60, 0, None);

    Output {
        syscfgen: apb2.en("syscfg", 0).en(),

        tim1en: apb2.en("tim1", 1).en(),
        spi1en: apb2.en("spi1", 2).en(),
        tim8en: apb2.en("tim8", 3).en(),
        usart1en: apb2.en("usart1", 4).en(),
        spi4en: apb2.en("spi4", 5).en(),
        tim15en: apb2.en("tim15", 6).en(),
        tim16en: apb2.en("tim16", 13).en(),
        tim17en: apb2.en("tim17", 14).en(),

        tim20en: apb2.en("tim20", 16).en(),
        sai1en: apb2.en("sai1", 17).en(),

        hrtim1en: apb2.en("hrtim1", 18).en(),
    }
}
