//! GPIO Port Input Data Register
//!
//! IDR registers contain fields which reflect the digital input state of the respective channels.

use phm::{
    Field, Register,
    field::access,
    model::{FieldEntry, PeripheralEntry, RegisterEntry},
};

/// GPIO peripherals implement this trait to attach Port Input Data registers.
pub trait Idr {
    /// Add a Port Input Data register to this peripheral.
    fn idr<'ncx>(&'ncx mut self, offset: u32) -> RegisterEntry<'ncx>;
}

impl<'cx> Idr for PeripheralEntry<'cx> {
    fn idr<'ncx>(&'ncx mut self, offset: u32) -> RegisterEntry<'ncx> {
        self.add_register(Register::new("idr", offset))
    }
}

/// Port Input Data registers implement this trait to attach Input Data fields.
pub trait Id {
    /// Add an Input Data field to this register.
    fn id<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Read>;
}

impl<'cx> Id for RegisterEntry<'cx> {
    fn id<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Read> {
        self.add_read_field(Field::new(format!("id{position}"), position, 1))
    }
}

pub mod id {
    use phm::{Variant, field::access, model::FieldEntry};

    /// Port Input Data fields implement this trait to be appropriately populated.
    pub trait Id {
        /// Populate the Port Input Data field with the appropriate contents.
        fn id(&mut self);
    }

    impl<'cx> Id for FieldEntry<'cx, access::Read> {
        fn id(&mut self) {
            self.add_variant(Variant::new("Low", 0));
            self.add_variant(Variant::new("High", 1));
        }
    }
}
