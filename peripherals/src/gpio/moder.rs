//! GPIO Port Mode Register
//!
//! MODER registers contain fields which are used to set the mode of the respective channels.
//!
//! Field topology:
//!
//! | Variant    | Bits |
//! | ---------- | ---- |
//! | Input      | 0    |
//! | Output     | 1    |
//! | Alternate  | 2    |
//! | Analog     | 3    |

use phm::{
    Field, Register,
    field::access,
    model::{FieldEntry, PeripheralEntry, RegisterEntry},
};

/// GPIO peripherals implement this trait to attach Port Mode registers.
pub trait Moder {
    /// Add a Port Mode register to this peripheral.
    fn moder<'ncx>(&'ncx mut self, offset: u32, reset: u32) -> RegisterEntry<'ncx>;
}

impl<'cx> Moder for PeripheralEntry<'cx> {
    fn moder<'ncx>(&'ncx mut self, offset: u32, reset: u32) -> RegisterEntry<'ncx> {
        self.add_register(Register::new("moder", offset).reset(reset))
    }
}

/// Port Mode registers implement this trait to attach Mode fields.
pub trait Mode {
    /// Add a Mode field to this register.
    fn mode<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store>;
}

impl<'cx> Mode for RegisterEntry<'cx> {
    fn mode<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store> {
        self.add_store_field(Field::new(format!("mode{position}"), position * 2, 2))
    }
}

pub type Output = [mode::Output; 16];

pub mod mode {
    use phm::{Entitlement, Variant, field::access, model::FieldEntry};

    /// Port Mode fields implement this trait to be appropriately populated.
    pub trait Mode {
        /// Populate the Port Mode field with the appropriate contents.
        fn mode(&mut self) -> Output;
    }

    impl<'cx> Mode for FieldEntry<'cx, access::Store> {
        fn mode(&mut self) -> Output {
            Output {
                input: self
                    .add_variant(Variant::new("Input", 0))
                    .make_entitlement(),
                output: self
                    .add_variant(Variant::new("Output", 1))
                    .make_entitlement(),
                alternate: self
                    .add_variant(Variant::new("Alternate", 2))
                    .make_entitlement(),
                analog: self
                    .add_variant(Variant::new("Analog", 3))
                    .make_entitlement(),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Output {
        pub input: Entitlement,
        pub output: Entitlement,
        pub alternate: Entitlement,
        pub analog: Entitlement,
    }
}
