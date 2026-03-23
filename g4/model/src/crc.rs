pub mod cr;
pub mod dr;
pub mod idr;
pub mod init;
pub mod pol;

use peripherals::rcc::enr::en;
use phm::{Composition, Peripheral};

use cr::cr;
use dr::dr;
use idr::idr;

use crate::crc::{init::init, pol::pol};

pub fn crc(composition: &mut Composition, crcen: en::Output) {
    let mut crc = composition.add_peripheral(Peripheral::new("crc", 0x4002_3000));
    crc.ontological_entitlements([[crcen.enabled]]);

    dr(&mut crc);
    idr(&mut crc);
    cr(&mut crc);
    init(&mut crc);
    pol(&mut crc);
}
