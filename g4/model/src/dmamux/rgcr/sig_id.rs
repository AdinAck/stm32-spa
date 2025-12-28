use proto_hal_model::{Field, model::RegisterEntry};

pub fn sig_id<'cx>(rgcr: &mut RegisterEntry<'cx>) {
    rgcr.add_store_field(Field::new("sig_id", 0, 5).docs(["Signal identification"]));
}
