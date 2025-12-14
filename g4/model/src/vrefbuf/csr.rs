pub mod envr;
pub mod hiz;
pub mod vrr;
pub mod vrs;

use proto_hal_model::{Register, model::PeripheralEntry};

use crate::vrefbuf::csr::{envr::envr, hiz::hiz, vrr::vrr, vrs::vrs};

pub fn csr<'cx>(vrefbuf: &mut PeripheralEntry<'cx>) {
    let mut csr = vrefbuf.add_register(Register::new("csr", 0).reset(0x2));

    envr(&mut csr);
    hiz(&mut csr);
    vrr(&mut csr);
    vrs(&mut csr);
}
