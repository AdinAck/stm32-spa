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

use model_macros::{field, register};
use phm::{Field, Register};

register! {
    /// Attach Port Pull-up/Pull-down registers to PGIO peripherals.
    Pupdr {
        /// Attach a Port Pull-up/Pull-down register to this peripheral.
        pupdr(offset: u32, reset: u32) {
            self.add_register(Register::new("pupdr", offset).reset(reset))
        }
    }
}

field! {
    /// Attach Pull-up/Pull-down fields to Port Pull-up/Pull-down registers.
    Pupd<Store> {
        /// Attach a Pull-up/Pull-down field to this register.
        pupd(position: u8) {
            self.add_store_field(Field::new_indexed(format!("pupd{position}"), position, 2))
        }
    }
}

/// A full Port Pull-up/Pull-down register. This means the register contains 16 Pull-up/Pull-down fields.
pub type Full = [pupd::PupdSchema; 16];

pub mod pupd {
    use model_macros::schema;
    use phm::Variant;

    schema! {
        /// Attach Pull-up/Pull-down schemas to Pull-up/Pull-down fields.
        Pupd<Store> {
            /// Pull-up/Pull-down.
            ///
            /// There are three pull-up/pull-down variants:
            /// - Floating (no pull-up or pull-down)
            /// - Pull Up
            /// - Pull Down
            pupd() {
                #[entitlement]
                floating: self.add_variant(Variant::new("Floating", 0)),
                #[entitlement]
                pullup: self.add_variant(Variant::new("PullUp", 1)),
                #[entitlement]
                pulldown: self.add_variant(Variant::new("PullDown", 2)),
            }
        }
    }
}
