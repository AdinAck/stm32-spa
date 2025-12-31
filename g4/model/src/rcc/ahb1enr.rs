use peripherals::{prelude::*, rcc::enr};
use proto_hal_model::{Field, model::PeripheralEntry};

pub struct Output {
    pub dma1en: enr::Output,
    pub dma2en: enr::Output,
    pub dmamux1en: enr::Output,
    pub cordicen: enr::Output,
    pub fmacen: enr::Output,

    pub flashen: enr::Output,

    pub crcen: enr::Output,
}

pub fn ahb1enr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut ahb1 = rcc.enr("ahb1", 0x48, 0x100, None);

    Output {
        dma1en: ahb1.en("dma1", 0),
        dma2en: ahb1.en("dma2", 1),
        dmamux1en: ahb1.en("dmamux1", 2),
        cordicen: ahb1.en("cordic", 3),
        fmacen: ahb1.en("fmac", 4),

        // TODO: RM0440 § 7.4.14: "This bit can be disabled only when the Flash is in power down mode."
        flashen: ahb1.en_from_field(Field::new("flash", 8, 1).leaky()),

        crcen: ahb1.en("crc", 12),
    }
}
