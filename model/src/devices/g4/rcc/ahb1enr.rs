use crate::peripherals::{prelude::*, rcc::enr::en};
use phm::model::PeripheralEntry;

pub struct Output {
    pub dma1en: en::EnSchema,
    pub dma2en: en::EnSchema,
    pub dmamux1en: en::EnSchema,
    pub cordicen: en::EnSchema,
    pub fmacen: en::EnSchema,

    pub flashen: en::EnSchema,

    pub crcen: en::EnSchema,
}

pub fn ahb1enr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut ahb1 = rcc.enr("ahb1", 0x48, 0x100, None);

    Output {
        dma1en: ahb1.en("dma1", 0).en(),
        dma2en: ahb1.en("dma2", 1).en(),
        dmamux1en: ahb1.en("dmamux1", 2).en(),
        cordicen: ahb1.en("cordic", 3).en(),
        fmacen: ahb1.en("fmac", 4).en(),

        // TODO: RM0440 § 7.4.14: "This bit can be disabled only when the Flash is in power down mode."
        flashen: ahb1.en("flash", 8).modify(|f| f.leaky()).en(),

        crcen: ahb1.en("crc", 12).en(),
    }
}
