use proto_hal_model::{Field, model::RegisterEntry};

use crate::dma::cpar;

pub fn pa8<'cx>(cpar: &mut RegisterEntry<'cx>, entitlements: cpar::Entitlements) {
    let mut pa8 = cpar.add_store_field(
        Field::new("pa8", 0, 32).docs(["Peripheral address for 8 bit transfers."]),
    );

    pa8.ontological_entitlements([entitlements.psize.bits8]);
    pa8.write_entitlements([entitlements.en.disabled]);
}
