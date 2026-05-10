//! GPIO Port Output Data Register
//!
//! ODR registers contain fields which reflect the digital output state of the respective channels.

use phm::{
    Field, Register,
    field::access,
    model::{FieldEntry, PeripheralEntry, RegisterEntry},
};

/// GPIO peripherals implement this trait to attach Port Output Data registers.
pub trait Odr {
    /// Add a Port Output Data register to this peripheral.
    fn odr<'ncx>(&'ncx mut self, offset: u32) -> RegisterEntry<'ncx>;
}

impl<'cx> Odr for PeripheralEntry<'cx> {
    fn odr<'ncx>(&'ncx mut self, offset: u32) -> RegisterEntry<'ncx> {
        self.add_register(Register::new("odr", offset).reset(0))
    }
}

/// Port Output Data registers implement this trait to attach Output Data fields.
pub trait Od {
    /// Add an Output Data field to this register.
    fn od<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store>;
}

impl<'cx> Od for RegisterEntry<'cx> {
    fn od<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store> {
        self.add_store_field(Field::new(format!("od{position}"), position, 1))
    }
}

pub type Output = [od::Output; 16];

pub mod od {
    use phm::{Entitlement, Variant, field::access, model::FieldEntry};

    /// Port Output Data fields implement this trait to be appropriately populated.
    pub trait Od {
        /// Populate the Port Output Data field with the appropriate contents.
        fn od(&mut self) -> Output;
    }

    impl<'cx> Od for FieldEntry<'cx, access::Store> {
        fn od(&mut self) -> Output {
            Output {
                low: self.add_variant(Variant::new("Low", 0)).make_entitlement(),
                high: self.add_variant(Variant::new("High", 1)).make_entitlement(),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Output {
        pub low: Entitlement,
        pub high: Entitlement,
    }
}
