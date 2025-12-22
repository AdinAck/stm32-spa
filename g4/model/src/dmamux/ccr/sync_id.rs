use proto_hal_model::{Field, model::RegisterEntry};

pub fn sync_id<'cx>(ccr: &mut RegisterEntry<'cx>) {
    ccr.add_store_field(Field::new("sync_id", 24, 5).docs(["Synchronization identification"]));
}
