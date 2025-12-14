use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn rt<'cx>(rtsr: &mut RegisterEntry<'cx>, i: u8, offset: u8) {
    let mut rt = rtsr.add_store_field(Field::new(format!("rt{i}"), offset, 1));

    rt.add_variant(Variant::new("Disabled", 0));
    rt.add_variant(Variant::new("Enabled", 1));
}
