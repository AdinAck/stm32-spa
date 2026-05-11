pub mod cgif;
pub mod chtif;
pub mod ctcif;
pub mod cteif;

use phm::{Register, model::PeripheralEntry};

use cgif::cgif;
use chtif::chtif;
use ctcif::ctcif;
use cteif::cteif;

pub fn ifcr<'cx>(dma: &mut PeripheralEntry<'cx>) {
    let mut ifcr = dma.add_register(Register::new("ifcr", 0x04));

    for i in 0..8 {
        cgif(&mut ifcr, i);
        ctcif(&mut ifcr, i);
        chtif(&mut ifcr, i);
        cteif(&mut ifcr, i);
    }
}
