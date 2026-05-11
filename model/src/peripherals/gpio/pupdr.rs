//! GPIO Port Pull-up/Pull-down Register
//!
//! PUPDR registers contain fields which are used to set the pull-up/pull-down of the respective channels.
//!
//! Field topology:
//!
//! | Variant  | Bits |
//! | -------- | ---- |
//! | Floating | 0    |
//! | PullUp   | 1    |
//! | PullDown | 2    |

use phm::{
    Field, Register,
    field::access,
    model::{FieldEntry, PeripheralEntry, RegisterEntry},
};

/// GPIO peripherals implement this trait to attach Port Pull-up/Pull-down registers.
pub trait Pupdr {
    /// Add a Port Pull-up/Pull-down register to this peripheral.
    fn pupdr<'ncx>(&'ncx mut self, offset: u32, reset: u32) -> RegisterEntry<'ncx>;
}

impl<'cx> Pupdr for PeripheralEntry<'cx> {
    fn pupdr<'ncx>(&'ncx mut self, offset: u32, reset: u32) -> RegisterEntry<'ncx> {
        self.add_register(Register::new("pupdr", offset).reset(reset))
    }
}

/// Port Pull-up/Pull-down registers implement this trait to attach Mode fields.
pub trait Pupd {
    /// Add a Pull-up/Pull-down field to this register.
    fn pupd<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store>;
}

impl<'cx> Pupd for RegisterEntry<'cx> {
    fn pupd<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store> {
        self.add_store_field(Field::new(format!("pupd{position}"), position * 2, 2))
    }
}

pub type Output = [pupd::Output; 16];

pub mod pupd {
    use phm::{Entitlement, Variant, field::access, model::FieldEntry};

    /// Port Pull-up/Pull-down fields implement this trait to be appropriately populated.
    pub trait Pupd {
        /// Populate the Port Pull-up/Pull-down field with the appropriate contents.
        fn pupd(&mut self) -> Output;
    }

    impl<'cx> Pupd for FieldEntry<'cx, access::Store> {
        fn pupd(&mut self) -> Output {
            Output {
                floating: self
                    .add_variant(Variant::new("Floating", 0))
                    .make_entitlement(),
                pullup: self
                    .add_variant(Variant::new("PullUp", 1))
                    .make_entitlement(),
                pulldown: self
                    .add_variant(Variant::new("PullDown", 2))
                    .make_entitlement(),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Output {
        pub floating: Entitlement,
        pub pullup: Entitlement,
        pub pulldown: Entitlement,
    }
}
