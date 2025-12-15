pub mod circ;
pub mod dir;
pub mod en;
pub mod htie;
pub mod mem2mem;
pub mod minc;
pub mod msize;
pub mod pinc;
pub mod pl;
pub mod psize;
pub mod tcie;
pub mod teie;

use proto_hal_model::{Register, model::PeripheralEntry};

use circ::circ;
use dir::dir;
use en::en;
use htie::htie;
use mem2mem::mem2mem;
use minc::minc;
use msize::msize;
use pinc::pinc;
use pl::pl;
use psize::psize;
use tcie::tcie;
use teie::teie;

pub struct Output {
    pub en: en::Output,
    pub psize: psize::Output,
    pub msize: msize::Output,
}

pub fn ccr<'cx>(dma: &mut PeripheralEntry<'cx>, channel: u8) -> Output {
    let mut ccr = dma.add_register(
        Register::new(format!("ccr{channel}"), 0x08 + 0x14 * channel as u32).reset(0),
    );

    let en = en(&mut ccr);
    tcie(&mut ccr, en);
    htie(&mut ccr, en);
    teie(&mut ccr, en);
    dir(&mut ccr, en);
    let circ = circ(&mut ccr, en);
    pinc(&mut ccr, en);
    minc(&mut ccr, en);
    let psize = psize(&mut ccr, en);
    let msize = msize(&mut ccr, en);
    pl(&mut ccr, en);
    mem2mem(&mut ccr, mem2mem::Entitlements { en, circ });

    Output { en, psize, msize }
}
