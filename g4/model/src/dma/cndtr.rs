pub mod ndt;

use proto_hal_model::{Register, model::PeripheralEntry};

use ndt::ndt;

use crate::dma::ccr::en;

pub struct Entitlements {
    pub en: en::Output,
}

pub fn cndtr<'cx>(dma: &mut PeripheralEntry<'cx>, channel: u8, entitlements: Entitlements) {
    let mut cndtr = dma.add_register(
        Register::new(format!("cndtr{channel}"), 0x0C + 0x14 * channel as u32).reset(0),
    );

    ndt(&mut cndtr, entitlements.en);
}
