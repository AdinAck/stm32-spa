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
    /// Attach Peripheral Clock Enable registers to Reset & Clock Control peripherals.
    Enr {
        /// Attach a Peripheral Clock Enable register to this peripheral.
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
    /// Attach Peripheral Clock Enable fields to Peripheral Clock Enable registers.
    En<Store> {
        /// Attach a Peripheral Clock Enable field to this register.
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
        /// Attach Peripheral Clock Enable schemas to Peripheral Clock Enable Fields.
        En<Store> {
            /// | Variant  | Bits |
            /// | -------- | ---- |
            /// | Disabled | 0    |
            /// | Enabled  | 1    |
            en() {
                /// The corresponding peripheral is disabled.
                #[entitlement]
                disabled: self.add_variant(Variant::new("Disabled", 0).docs(["Peripheral clock is disabled."])),

                /// The corresponding peripheral is enabled.
                ///
                /// *Note: On some devices there is a delay (usually measured in number of cycles on the respective bus
                /// ) between this variant being set and the peripheral actually being enabled.*
                #[entitlement]
                enabled: self.add_variant(Variant::new("Enabled", 1).docs(["Peripheral clock is enabled."]))
            }
        }
    }
}
