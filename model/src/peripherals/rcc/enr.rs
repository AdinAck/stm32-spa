//! Peripheral Clock Enable Register
//!
//! ENR registers contain fields which are used to enable and disable peripheral clock domains. Peripherals named by
//! such fields possess ontological entitlements to the `Enabled` state of their respective clock enable field.
//!
//! Field topology:
//!
//! | Variant  | Bits |
//! | -------- | ---- |
//! | Disabled | 0    |
//! | Enabled  | 1    |

use model_macros::{field, register};
use phm::{Field, Register};

register! {
    /// Reset & Clock Control peripherals implement this trait to attach Peripheral Clock Enable registers.
    Enr {
        /// Add a Peripheral Clock Enable register to this peripheral.
        ///
        /// *Note: The suffix "enr" is added to the provided name.*
        enr(
            name: impl AsRef<str>,
            offset: u32,
            reset: u32,
            index: Option<usize>,
        ) {
            let name = name.as_ref();

            let title = "peripheral clock enable register";

            self.add_register(Register::new(if let Some(index) = index {
                format!("{name}enr{index}")
            } else {
                format!("{name}enr")
            }, offset).reset(reset).docs([if let Some(index) = index {
                format!("{name} {title} {index}.")
            } else {
                format!("{name} {title}.")
            }]))
        }
    }
}

field! {
    /// Peripheral Clock Enable registers implement this trait to attach Peripheral Clock Enable fields.
    En<Store> {
        /// Add a Peripheral Clock Enable field to this register.
        ///
        /// *Note: The suffix "en" is added to the provided name.*
        en(name: impl AsRef<str>, position: u8) {
            let name = name.as_ref();

            self.add_store_field(
                Field::new(format!("{name}en"), position, 1).docs([format!("{name} clock enable.")]),
            )
        }
    }
}

pub mod en {
    use model_macros::schema;
    use phm::Variant;

    schema! {
        En<Store> {
            /// | Variant  | Bits |
            /// | -------- | ---- |
            /// | Disabled | 0    |
            /// | Enabled  | 1    |
            en() {
                #[entitlement]
                disabled: self.add_variant(Variant::new("Disabled", 0).docs(["Peripheral clock is disabled."])),
                #[entitlement]
                enabled: self.add_variant(Variant::new("Enabled", 1).docs(["Peripheral clock is enabled."]))
            }
        }
    }
}
