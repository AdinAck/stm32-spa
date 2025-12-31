use peripherals::{prelude::*, rcc::enr};
use proto_hal_model::{Field, model::PeripheralEntry};

pub struct Output {
    pub dma1en: enr::Output,

    pub flashen: enr::Output,

    pub crcen: enr::Output,
}

pub fn ahbenr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut ahb = rcc.enr("ahb", 0x38, 0x100, None);

    Output {
        dma1en: ahb.en("dma1", 0),

        // TODO: RM0490 § 6.4.13: "This bit can only be cleared when the flash memory is in power down mode."
        flashen: ahb.en_from_field(Field::new("flash", 8, 1).leaky()),

        crcen: ahb.en("crc", 12),
    }
}
