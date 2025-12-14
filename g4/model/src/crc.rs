pub mod cr;
pub mod dr;
pub mod idr;
pub mod init;
pub mod pol;

use proto_hal_model::{Entitlement, Model, Peripheral};

use cr::cr;
use dr::dr;
use idr::idr;

use crate::crc::{init::init, pol::pol};

pub fn crc(model: &mut Model, crcen: Entitlement) {
    let mut crc = model.add_peripheral(Peripheral::new("crc", 0x4002_3000));
    crc.ontological_entitlements([crcen]);

    dr(&mut crc);
    idr(&mut crc);
    cr(&mut crc);
    init(&mut crc);
    pol(&mut crc);
}
