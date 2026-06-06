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

use model_macros::{field, register};
use phm::{Field, Register};

register! {
    /// Attach Port Output Type registers to GPIO peripherals.
    Otyper {
        /// Attach a Port Output Type register to this peripheral.
        otyper(offset: u32, reset: u32) {
            self.add_register(Register::new("otyper", offset).reset(reset))
        }
    }
}

field! {
    /// Attach Output Type fields to Port Output Type registers.
    Ot<Store> {
        /// Attach an Output Type field to this register.
        ot(position: u8) {
            self.add_store_field(Field::new(format!("ot{position}"), position, 1))
        }
    }
}

/// A full Port Output Type register. This means the register contains 16 Output Type fields.
pub type Full = [ot::OtSchema; 16];

pub mod ot {
    use model_macros::schema;
    use phm::Variant;

    schema! {
        /// Attach an Output Type schema to an Output Type field.
        Ot<Store> {
            /// Output Type.
            ///
            /// There are two output types:
            /// - Push-Pull
            /// - Open Drain
            ot() {
                #[entitlement]
                push_pull: self.add_variant(Variant::new("PushPull", 0)),
                #[entitlement]
                open_drain: self.add_variant(Variant::new("OpenDrain", 1)),
            }
        }
    }
}
