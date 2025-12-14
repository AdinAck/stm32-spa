pub mod ahb1;
pub mod ahb2;
pub mod apb2;

use proto_hal_model::{Entitlement, Field, Variant, model::RegisterEntry};

fn add_field<'cx>(reg: &mut RegisterEntry<'cx>, field: Field) -> Entitlement {
    let mut en = reg.add_store_field(field);

    en.add_variant(Variant::new("Disabled", 0));
    en.add_variant(Variant::new("Enabled", 1))
        .make_entitlement()
}
