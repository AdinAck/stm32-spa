pub mod ge;
pub mod gnbreq;
pub mod gpol;
pub mod oie;
pub mod sig_id;

use proto_hal_model::{Register, model::PeripheralEntry};

use crate::dmamux::rgcr::{ge::ge, gnbreq::gnbreq, gpol::gpol, oie::oie, sig_id::sig_id};

pub fn rgcr<'cx>(dmamux: &mut PeripheralEntry<'cx>, channel: u8) {
    let mut rgcr = dmamux
        .add_register(Register::new(format!("rg{channel}cr"), 0x100 + channel as u32 * 4).reset(0));

    sig_id(&mut rgcr);
    oie(&mut rgcr);
    let ge = ge(&mut rgcr);
    gpol(&mut rgcr);
    gnbreq(&mut rgcr, ge.disabled);
}
