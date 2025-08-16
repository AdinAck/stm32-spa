use phm::{Field, Variant, model::RegisterEntry};

use crate::dma::ccr::en;

pub fn minc<'cx>(ccr: &mut RegisterEntry<'cx>, en: en::Output) -> phm::Result<()> {
    let mut minc = ccr.add_store_field(Field::new("minc", 7, 1).docs(["Memory increment mode"]));

    minc.write_entitlements([[en.disabled]])?;

    minc.add_variant(Variant::new("Disabled", 0));
    minc.add_variant(Variant::new("Enabled", 1));

    Ok(())
}
