//! GPIO Port Output Type Register
//!
//! OTYPER registers contain fields which are used to set the output type of the respective channels.
//!
//! Field topology:
//!
//! | Variant    | Bits |
//! | ---------- | ---- |
//! | Push-pull  | 0    |
//! | Open-drain | 1    |

use phm::{
    Field, Register,
    field::access,
    model::{FieldEntry, PeripheralEntry, RegisterEntry},
};

/// GPIO peripherals implement this trait to attach Port Output Type registers.
pub trait Otyper {
    /// Add a Port Output Type register to this peripheral.
    fn otyper<'ncx>(&'ncx mut self, offset: u32, reset: u32) -> RegisterEntry<'ncx>;
}

impl<'cx> Otyper for PeripheralEntry<'cx> {
    fn otyper<'ncx>(&'ncx mut self, offset: u32, reset: u32) -> RegisterEntry<'ncx> {
        self.add_register(Register::new("otyper", offset).reset(reset))
    }
}

/// Port Output Type registers implement this trait to attach Output Type fields.
pub trait Ot {
    /// Add an Output Type field to this register.
    fn ot<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store>;
}

impl<'cx> Ot for RegisterEntry<'cx> {
    fn ot<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store> {
        self.add_store_field(Field::new(format!("ot{position}"), position, 1))
    }
}

pub type Output = [ot::Output; 16];

pub mod ot {
    use phm::{Entitlement, Variant, field::access, model::FieldEntry};

    /// Port Output Type fields implement this trait to be appropriately populated.
    pub trait Ot {
        /// Populate the Port Output Type field with the appropriate contents.
        fn ot(&mut self) -> Output;
    }

    impl<'cx> Ot for FieldEntry<'cx, access::Store> {
        fn ot(&mut self) -> Output {
            Output {
                push_pull: self
                    .add_variant(Variant::new("PushPull", 0))
                    .make_entitlement(),
                open_drain: self
                    .add_variant(Variant::new("OpenDrain", 1))
                    .make_entitlement(),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Output {
        pub push_pull: Entitlement,
        pub open_drain: Entitlement,
    }
}
