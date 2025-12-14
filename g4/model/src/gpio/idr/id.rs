use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn id<'cx>(idr: &mut RegisterEntry<'cx>, i: u8) {
    let mut id = idr.add_read_field(Field::new(format!("id{i}"), i, 1));

    id.add_variant(Variant::new("Low", 0));
    id.add_variant(Variant::new("High", 1));
}
