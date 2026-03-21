use peripherals::{prelude::*, rcc::enr::en};
use phm::model::PeripheralEntry;

pub struct Output {
    pub dma1en: en::Output,

    pub flashen: en::Output,

    pub crcen: en::Output,
}

pub fn ahbenr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut ahb = rcc.enr("ahb", 0x38, 0x100, None);

    Output {
        dma1en: ahb.en("dma1", 0).en(),

        // TODO: RM0490 § 6.4.13: "This bit can only be cleared when the flash memory is in power down mode."
        flashen: ahb.en("flash", 8).modify(|f| f.leaky()).en(),

        crcen: ahb.en("crc", 12).en(),
    }
}
