//! GPIO Port Output Data Register
//!
//! ODR registers contain fields which reflect the digital output state of the respective channels.

use model_macros::{field, register};
use phm::{Field, Register};

register! {
    /// Attach Port Output Data registers to GPIO peripherals.
    Odr {
        /// Attach a Port Output Data register to this peripheral.
        odr(offset: u32) {
            self.add_register(Register::new("odr", offset).reset(0))
        }
    }
}

field! {
    /// Attach Output Data fields to Port Output Data registers.
    Od<Store> {
        /// Attach an Output Data field to this register.
        od(position: u8) {
            self.add_store_field(Field::new(format!("od{position}"), position, 1))
        }
    }
}

/// A full Port Output Data register. This means the register contains 16 Output data fields.
pub type Full = [od::OdSchema; 16];

pub mod od {
    use model_macros::schema;
    use phm::Variant;

    schema! {
        /// Attach Output Data schemas to Output Data fields.
        Od<Store> {
            /// Output Data. The configured channel level (either LOW or HIGH).
            od() {
                #[entitlement]
                /// The configured channel level is LOW.
                low: self.add_variant(Variant::new("Low", 0)),
                #[entitlement]
                /// The configured channel level is HIGH.
                high: self.add_variant(Variant::new("High", 1)),
            }
        }
    }
}
