use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn ot<'cx>(otyper: &mut RegisterEntry<'cx>, i: u8) {
    let mut ot = otyper.add_store_field(Field::new(format!("ot{i}"), i, 1));

    ot.add_variant(Variant::new("PushPull", 0));
    ot.add_variant(Variant::new("OpenDrain", 1));
}
