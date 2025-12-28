pub mod ma16;
pub mod ma32;
pub mod ma8;

use proto_hal_model::{Register, model::PeripheralEntry};

use crate::dma::{
    ccr::{en, msize},
    cmar::{ma8::ma8, ma16::ma16, ma32::ma32},
};

#[derive(Debug, Clone, Copy)]
pub struct Entitlements {
    pub en: en::Output,
    pub msize: msize::Output,
}

pub fn cmar<'cx>(dma: &mut PeripheralEntry<'cx>, channel: u8, entitlements: Entitlements) {
    let mut cmar = dma.add_register(
        Register::new(format!("cmar{channel}"), 0x14 + 0x14 * channel as u32).reset(0),
    );

    ma8(&mut cmar, entitlements);
    ma16(&mut cmar, entitlements);
    ma32(&mut cmar, entitlements);
}
