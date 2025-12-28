use proto_hal_model::{Field, model::RegisterEntry};

use crate::dmamux::ccr::{ege, se};

pub struct Entitlements {
    pub ege: ege::Output,
    pub se: se::Output,
}

pub fn nbreq<'cx>(ccr: &mut RegisterEntry<'cx>, entitlements: Entitlements) {
    let mut nbreq = ccr.add_store_field(
        Field::new("nbreq", 19, 5).docs(["Number of DMA requests minus 1 to forward"]),
    );

    nbreq.write_entitlements([entitlements.ege.disabled, entitlements.se.disabled]);
}
