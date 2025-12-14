use proto_hal_model::{Field, Register, model::PeripheralEntry};

pub fn init<'cx>(crc: &mut PeripheralEntry<'cx>) {
    let mut init = crc.add_register(Register::new("init", 0x10).reset(u32::MAX));

    init.add_store_field(Field::new("init", 0, 32));
}
