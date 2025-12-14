use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn im<'cx>(imr: &mut RegisterEntry<'cx>, i: u8, offset: u8) {
    let mut im = imr.add_store_field(Field::new(format!("im{i}"), offset, 1));

    im.add_variant(Variant::new("Masked", 0));
    im.add_variant(Variant::new("Unmasked", 1));
}
