use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn soie<'cx>(ccr: &mut RegisterEntry<'cx>) {
    let mut soie = ccr.add_store_field(
        Field::new("soie", 8, 1).docs(["Synchronization overrun interrupt enable"]),
    );

    soie.add_variant(Variant::new("Disabled", 0));
    soie.add_variant(Variant::new("Enabled", 1));
}
