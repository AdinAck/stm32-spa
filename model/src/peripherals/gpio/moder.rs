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

use model_macros::{field, register};
use phm::{Field, Register};

register! {
    /// Attach Port Mode registers to GPIO peripherals.
    Moder {
        /// Attach a Port Mode register to this peripheral.
        moder(offset: u32, reset: u32) {
            self.add_register(Register::new("moder", offset).reset(reset))
        }
    }
}

field! {
    /// Attach Mode fields to Port Mode registers.
    Mode<Store> {
        /// Attach a Mode field to this register.
        mode(position: u8) {
            self.add_store_field(Field::new(format!("mode{position}"), position * 2, 2))
        }
    }
}

/// A full Port Mode register. This means the register contains 16 mode fields.
pub type Full = [mode::ModeSchema; 16];

pub mod mode {
    use model_macros::schema;
    use phm::Variant;

    schema! {
        /// Attach Mode schemas to Mode fields.
        Mode<Store> {
            /// Mode.
            ///
            /// There are 4 modes:
            /// 1. Input
            /// 1. Output
            /// 1. Alternate
            /// 1. Analog
            mode() {
                #[entitlement]
                input: self.add_variant(Variant::new("Input", 0)),
                #[entitlement]
                output: self.add_variant(Variant::new("Output", 1)),
                #[entitlement]
                alternate: self.add_variant(Variant::new("Alternate", 2)),
                #[entitlement]
                analog: self.add_variant(Variant::new("Analog", 3)),
            }
        }
    }
}
