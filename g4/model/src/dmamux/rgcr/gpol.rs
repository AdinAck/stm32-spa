use proto_hal_model::{Field, Variant, model::RegisterEntry};

pub fn gpol<'cx>(rgcr: &mut RegisterEntry<'cx>) {
    let mut gpol = rgcr.add_store_field(
        Field::new("gpol", 17, 2).docs(["DMA request generator trigger polarity"]),
    );

    gpol.add_variant(Variant::new("NoEvent", 0));
    gpol.add_variant(Variant::new("RisingEdge", 1));
    gpol.add_variant(Variant::new("FallingEdge", 2));
    gpol.add_variant(Variant::new("RisingAndFallingEdges", 3));
}
