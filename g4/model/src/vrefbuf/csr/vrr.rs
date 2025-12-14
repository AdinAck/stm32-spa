use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn vrr<'cx>(csr: &mut RegisterEntry<'cx>) {
    let mut vrr = csr.add_read_field(Field::new("vrr", 3, 1));

    vrr.add_variant(Variant::new("NotReady", 0));
    vrr.add_variant(Variant::new("Ready", 1));
}
