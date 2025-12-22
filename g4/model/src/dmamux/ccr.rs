use proto_hal_model::{Register, model::PeripheralEntry};

use crate::dmamux::ccr::{
    dmareq::dmareq, ege::ege, nbreq::nbreq, se::se, soie::soie, spol::spol, sync_id::sync_id,
};

pub mod dmareq;
pub mod ege;
pub mod nbreq;
pub mod se;
pub mod soie;
pub mod spol;
pub mod sync_id;

pub fn ccr<'cx>(dmamux: &mut PeripheralEntry<'cx>, channel: u8) {
    let mut ccr =
        dmamux.add_register(Register::new(format!("c{channel}cr"), channel as u32 * 4).reset(0));

    dmareq(&mut ccr);
    soie(&mut ccr);
    let ege = ege(&mut ccr);
    let se = se(&mut ccr);
    spol(&mut ccr);
    nbreq(&mut ccr, nbreq::Entitlements { ege, se });
    sync_id(&mut ccr);
}
