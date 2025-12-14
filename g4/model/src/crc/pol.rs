use proto_hal_model::{Field, Register, model::PeripheralEntry};

pub fn pol<'cx>(crc: &mut PeripheralEntry<'cx>) {
    let mut pol = crc.add_register(Register::new("pol", 0x14).reset(0x04c1_1db7));

    pol.add_store_field(Field::new("pol", 0, 32));
}
