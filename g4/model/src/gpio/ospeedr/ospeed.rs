use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn ospeed<'cx>(ospeedr: &mut RegisterEntry<'cx>, i: u8) {
    let mut ospeed = ospeedr.add_store_field(Field::new(format!("ospeed{i}"), i * 2, 2));

    ospeed.add_variant(Variant::new("Low", 0));
    ospeed.add_variant(Variant::new("Medium", 1));
    ospeed.add_variant(Variant::new("High", 2));
    ospeed.add_variant(Variant::new("VeryHigh", 3));
}
