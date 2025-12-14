use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn exti<'cx>(exticr: &mut RegisterEntry<'cx>, i: u8) {
    let mut exti = exticr.add_store_field(Field::new(format!("exti{i}"), (i % 4) * 4, 4));

    exti.add_variant(Variant::new("PA", 0));
    exti.add_variant(Variant::new("PB", 1));
    exti.add_variant(Variant::new("PC", 2));
    exti.add_variant(Variant::new("PD", 3));
    exti.add_variant(Variant::new("PE", 4));
    exti.add_variant(Variant::new("PF", 5));
    exti.add_variant(Variant::new("PG", 6));
}
