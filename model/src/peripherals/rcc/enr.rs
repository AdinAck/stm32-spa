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

use phm::{
    Field, Register,
    field::access,
    model::{FieldEntry, PeripheralEntry, RegisterEntry},
};
use quote::format_ident;

/// Reset & Clock Control peripherals implement this trait to attach Peripheral Clock Enable registers.
pub trait Enr {
    /// Add a Peripheral Clock Enable register to this peripheral.
    ///
    /// *Note: The suffix "enr" is added to the provided name.*
    fn enr<'ncx>(
        &'ncx mut self,
        name: impl AsRef<str>,
        offset: u32,
        reset: u32,
        index: Option<usize>,
    ) -> RegisterEntry<'ncx> {
        let name = name.as_ref();

        self.enr_from_register(Register::new(name, offset).reset(reset), index)
    }

    /// Add a Peripheral Clock Enable register to this peripheral given a [`Register`] component.
    ///
    /// *Note: The suffix "enr" is added to the provided name.*
    fn enr_from_register<'ncx>(
        &'ncx mut self,
        register: Register,
        index: Option<usize>,
    ) -> RegisterEntry<'ncx>;
}

impl<'cx> Enr for PeripheralEntry<'cx> {
    fn enr_from_register<'ncx>(
        &'ncx mut self,
        mut register: Register,
        index: Option<usize>,
    ) -> RegisterEntry<'ncx> {
        let name = register.module_name();

        if let Some(index) = index {
            register.ident = format_ident!("{name}enr{index}");
        } else {
            register.ident = format_ident!("{name}enr");
        }

        let title = "peripheral clock enable register";

        self.add_register(register.docs([if let Some(index) = index {
            format!("{name} {title} {index}.")
        } else {
            format!("{name} {title}.")
        }]))
    }
}

/// Peripheral Clock Enable registers implement this trait to attach Peripheral Clock Enable fields.
pub trait En {
    /// Add a Peripheral Clock Enable field to this register.
    ///
    /// *Note: The suffix "en" is added to the provided name.*
    fn en<'ncx>(
        &'ncx mut self,
        name: impl AsRef<str>,
        position: u8,
    ) -> FieldEntry<'ncx, access::Store>;
}

impl<'cx> En for RegisterEntry<'cx> {
    fn en<'ncx>(
        &'ncx mut self,
        name: impl AsRef<str>,
        position: u8,
    ) -> FieldEntry<'ncx, access::Store> {
        let name = name.as_ref();

        self.add_store_field(
            Field::new(format!("{name}en"), position, 1).docs([format!("{name} clock enable.")]),
        )
    }
}

pub mod en {
    use phm::{Entitlement, Variant, field::access, model::FieldEntry};

    /// Peripheral Clock Enable fields implement this trait to be appropriately populated.
    pub trait En {
        /// Populate the Peripheral Clock Enable field with the appropriate contents.
        fn en(&mut self) -> Output;
    }

    impl<'cx> En for FieldEntry<'cx, access::Store> {
        fn en(&mut self) -> Output {
            Output {
                disabled: self
                    .add_variant(
                        Variant::new("Disabled", 0).docs(["Peripheral clock is disabled."]),
                    )
                    .make_entitlement(),
                enabled: self
                    .add_variant(Variant::new("Enabled", 1).docs(["Peripheral clock is enabled."]))
                    .make_entitlement(),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Output {
        pub disabled: Entitlement,
        pub enabled: Entitlement,
    }
}
