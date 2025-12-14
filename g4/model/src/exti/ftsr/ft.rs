use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn ft<'cx>(ftsr: &mut RegisterEntry<'cx>, i: u8, offset: u8) {
    let mut ft = ftsr.add_store_field(Field::new(format!("ft{i}"), offset, 1));

    ft.add_variant(Variant::new("Disabled", 0));
    ft.add_variant(Variant::new("Enabled", 1));
}
