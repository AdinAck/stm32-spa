use proto_hal_model::{Entitlement, Field, model::RegisterEntry};

pub fn gnbreq<'cx>(rgcr: &mut RegisterEntry<'cx>, ge_disabled: Entitlement) {
    let mut gnbreq = rgcr.add_store_field(
        Field::new("gnbreq", 19, 5).docs(["Number of DMA requests to be generated (minus 1)"]),
    );

    gnbreq.write_entitlements([ge_disabled]);
}
