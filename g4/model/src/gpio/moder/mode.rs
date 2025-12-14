use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn mode<'cx>(moder: &mut RegisterEntry<'cx>, i: u8) {
    let mut mode = moder.add_store_field(Field::new(format!("mode{i}"), i * 2, 2));

    mode.add_variant(Variant::new("Input", 0));
    mode.add_variant(Variant::new("Output", 1));
    mode.add_variant(Variant::new("Alternate", 2));
    mode.add_variant(Variant::new("Analog", 3));
}
