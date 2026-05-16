//! GPIO Port Input Data Register
//!
//! IDR registers contain fields which reflect the digital input state of the respective channels.

use model_macros::{field, register};
use phm::{Field, Register};

register! {
    /// GPIO peripherals implement this trait to attach Port Input Data registers.
    Idr {
        /// Attach a Port Input Data register to this peripheral.
        idr(offset: u32) {
            self.add_register(Register::new("idr", offset))
        }
    }
}

field! {
    /// Attach Input Data fields to Port Input Data registers.
    Id<Read> {
        /// Attach an Input Data field to this register.
        id(position: u8) {
            self.add_read_field(Field::new(format!("id{position}"), position, 1))
        }
    }
}

pub mod id {
    use model_macros::schema;
    use phm::Variant;

    schema! {
        /// Attach Input Data schemas to Input Data fields.
        Id<Read> {
            /// Input Data. The measured channel level (either LOW or HIGH).
            id() {
                /// The measured channel level is LOW.
                low: self.add_variant(Variant::new("Low", 0)),
                /// The measured channel level is HIGH.
                high: self.add_variant(Variant::new("High", 1)),
            }
        }
    }
}
