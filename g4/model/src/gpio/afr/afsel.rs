use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn afsel<'cx>(afr: &mut RegisterEntry<'cx>, i: u8) {
    let mut afsel = afr.add_store_field(Field::new(format!("afsel{i}"), (i % 8) * 4, 4));

    for v in 0..16 {
        afsel.add_variant(Variant::new(format!("AF{v}"), v));
    }
}
