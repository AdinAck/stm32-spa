//! GPIO Alternate Function Register
//!
//! AFRx registers contain fields which are used to set the alternate function of the respective channels.
//!
//! Each Alternate Function Select field contains 16 variants AF0 through AF15.

use model_macros::{field, register};
use phm::{Field, Register};

register! {
    /// Attach Alternate Function registers to GPIO peripherals.
    Afr {
        /// Attach a lower Alternate Function register to this peripheral.
        afrl(offset: u32) {
            self.add_register(Register::new("afrl", offset).reset(0))
        }
        /// Attach an upper Alternate Function register to this peripheral.
        afrh(offset: u32) {
            self.add_register(Register::new("afrh", offset).reset(0))
        }
    }
}

field! {
    /// Attach Alternate Function Selection fields to Alternate Function registers.
    Afsel<Store> {
        /// Attach an Alternate Function Selection field to this register.
        ///
        /// *Note: "position" is not the bit offset, but rather the index of the field.*
        afsel(position: u8) {
            self.add_store_field(Field::new_indexed(
                format!("afsel{position}"),
                position,
                4,
            ))
        }
    }
}

/// A full Alternate Function Selection register. This means the register contains 8 Alternate Function Selection fields.
pub type Full = [afsel::AfselSchema; 8];

pub mod afsel {
    use model_macros::schema;
    use phm::Variant;

    schema! {
        /// Attach Alternate Function Selection schemas to Alternate Function Selection fields.
        Afsel<Store> {
            /// Alternate Function Selection.
            ///
            /// There are 16 alternate functions (AF0-AF15).
            afsel() {
                /// A selected alternate function.
                #[entitlement]
                #[array(0..16, index_pattern = "x")]
                afx: self.add_variant(Variant::new(format!("AF{x}"), x as _)),
            }
        }
    }
}
