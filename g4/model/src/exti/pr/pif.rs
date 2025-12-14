use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn pif<'cx>(pr: &mut RegisterEntry<'cx>, i: u8, offset: u8) {
    let mut pif = pr.add_read_write_field(Field::new(format!("pif{i}"), offset, 1));

    pif.add_read_variant(Variant::new("Idle", 0));
    pif.add_read_variant(Variant::new("Pending", 1));

    pif.add_write_variant(Variant::new("Noop", 0).inert());
    pif.add_write_variant(Variant::new("Clear", 1));
}
