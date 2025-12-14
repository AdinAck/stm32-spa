use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn em<'cx>(emr: &mut RegisterEntry<'cx>, i: u8, offset: u8) {
    let mut em = emr.add_store_field(Field::new(format!("em{i}"), offset, 1));

    em.add_variant(Variant::new("Masked", 0));
    em.add_variant(Variant::new("Unmasked", 1));
}
