use proto_hal_model::{Entitlement, Field, Variant, model::RegisterEntry};

use crate::dma::ccr::en;

#[derive(Debug, Clone, Copy)]
pub struct Output {
    pub bits8: Entitlement,
    pub bits16: Entitlement,
    pub bits32: Entitlement,
}

pub fn msize<'cx>(ccr: &mut RegisterEntry<'cx>, en: en::Output) -> Output {
    let mut msize = ccr.add_store_field(Field::new("msize", 10, 2).docs(["Memory size"]));

    msize.write_entitlements([en.disabled]);

    let bits8 = msize
        .add_variant(Variant::new("Bits8", 0))
        .make_entitlement();
    let bits16 = msize
        .add_variant(Variant::new("Bits16", 1))
        .make_entitlement();
    let bits32 = msize
        .add_variant(Variant::new("Bits32", 2))
        .make_entitlement();

    Output {
        bits8,
        bits16,
        bits32,
    }
}
