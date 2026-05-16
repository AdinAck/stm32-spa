//! GPIO Port Output Speed Register
//!
//! OSPEEDR registers contain fields which are used to set the output speed of the respective channels.
//!
//! Field topology:
//!
//! | Variant  | Bits |
//! | -------- | ---- |
//! | Low      | 0    |
//! | Medium   | 1    |
//! | High     | 2    |
//! | VeryHigh | 3    |

use model_macros::{field, register};
use phm::{Field, Register};

register! {
    /// Attach Port Output Speed registers to GPIO peripherals.
    Ospeedr {
        /// Attach a Port Output Speed register to this peripheral.
        ospeedr(offset: u32, reset: u32) {
            self.add_register(Register::new("ospeedr", offset).reset(reset))
        }
    }
}

field! {
    /// Attach Output Speed fields to Port Output Speed registers.
    Ospeed<Store> {
        /// Attach an Output Speed field to this register.
        ospeed(position: u8) {
            self.add_store_field(Field::new(format!("ospeed{position}"), position * 2, 2))
        }
    }
}

/// A full Poty Output Speed register. This means the register contains 16 Output Speed fields.
pub type Full = [ospeed::OspeedSchema; 16];
/// A full Port Output Speed register. This means the register contains 16 Output Speed fields (with
/// [very high speed](ospeed::Ospeed::ospeed_with_very_high)).
pub type FullWithVeryHigh = [ospeed::OspeedWithVeryHighSchema; 16];

pub mod ospeed {
    use model_macros::schema;
    use phm::Variant;

    schema! {
        /// Attach Output Speed schemas to Output Speed fields.
        Ospeed<Store> {
            /// Output Speed.
            ///
            /// This schema inherts from [`ospeed`](Ospeed::ospeed) and adds the `VeryHigh` speed variant.
            ///
            /// Some channels are incapable of this speed (*RM0490 § 8.5.3*).
            #[inherits(ospeed)]
            ospeed_with_very_high() {
                /// Very high speed.
                #[entitlement]
                very_high: self.add_variant(Variant::new("VeryHigh", 3)),
            }

            /// Output Speed.
            ospeed() {
                /// Low speed.
                #[entitlement]
                low: self.add_variant(Variant::new("Low", 0)),
                /// Medium speed.
                #[entitlement]
                medium: self.add_variant(Variant::new("Medium", 1)),
                /// High speed.
                #[entitlement]
                high: self.add_variant(Variant::new("High", 2)),
            }
        }
    }
}
