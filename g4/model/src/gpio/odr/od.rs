use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn od<'cx>(odr: &mut RegisterEntry<'cx>, i: u8) {
    let mut od = odr.add_store_field(Field::new(format!("od{i}"), i, 1));

    od.add_variant(Variant::new("Low", 0));
    od.add_variant(Variant::new("High", 1));
}
