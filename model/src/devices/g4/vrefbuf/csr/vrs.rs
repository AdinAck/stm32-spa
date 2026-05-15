use phm::{Field, Variant, model::RegisterEntry};

use crate::devices::g4::vrefbuf::csr::envr;

pub fn vrs<'cx>(csr: &mut RegisterEntry<'cx>, envr: &envr::Output) {
    let mut vrs = csr.add_store_field(Field::new("vrs", 4, 2));

    vrs.write_entitlements([[envr.disabled]]);

    vrs.add_variant(Variant::new("R2V048", 0).docs(["2.048v."]));
    vrs.add_variant(Variant::new("R2V500", 1).docs(["2.500v"]));
    vrs.add_variant(Variant::new("R2V900", 2).docs(["2.900v"]));
}
