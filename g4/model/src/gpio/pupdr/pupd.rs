use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn pupd<'cx>(pupdr: &mut RegisterEntry<'cx>, i: u8) {
    let mut pupd = pupdr.add_store_field(Field::new(format!("pupd{i}"), i * 2, 2));

    pupd.add_variant(Variant::new("Floating", 0));
    pupd.add_variant(Variant::new("PullUp", 1));
    pupd.add_variant(Variant::new("PullDown", 2));
}
