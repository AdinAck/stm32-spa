pub mod pa16;
pub mod pa32;
pub mod pa8;

use proto_hal_model::{Register, model::PeripheralEntry};

use crate::dma::{
    ccr::{en, psize},
    cpar::{pa8::pa8, pa16::pa16, pa32::pa32},
};

#[derive(Debug, Clone, Copy)]
pub struct Entitlements {
    pub en: en::Output,
    pub psize: psize::Output,
}

pub fn cpar<'cx>(dma: &mut PeripheralEntry<'cx>, channel: u8, entitlements: Entitlements) {
    let mut cpar = dma.add_register(
        Register::new(format!("cpar{channel}"), 0x10 + 0x14 * channel as u32).reset(0),
    );

    pa8(&mut cpar, entitlements);
    pa16(&mut cpar, entitlements);
    pa32(&mut cpar, entitlements);
}
