use phm::{Field, model::RegisterEntry};

use crate::dma::cpar;

pub fn pa32<'cx>(
    cpar: &mut RegisterEntry<'cx>,
    entitlements: cpar::Entitlements,
) -> phm::Result<()> {
    let mut pa32 = cpar.add_store_field(
        Field::new("pa32", 0, 30).docs(["Peripheral address for 32 bit transfers."]),
    );

    pa32.ontological_entitlements([[entitlements.psize.bits32]])?;
    pa32.write_entitlements([[entitlements.en.disabled]])?;

    Ok(())
}
