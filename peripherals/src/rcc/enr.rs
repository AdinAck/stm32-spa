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

use proto_hal_model::{
    Entitlement, Field, Register, Variant,
    model::{PeripheralEntry, RegisterEntry},
};
use quote::format_ident;

/// Reset & Clock Control peripherals implement this trait.
pub trait Rcc {
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

    /// Add a Peripheral Clock Enable register to this peripheral given a [`Regsiter`] component.
    ///
    /// *Note: The suffix "enr" is added to the provided name.*
    fn enr_from_register<'cx>(
        &'cx mut self,
        register: Register,
        index: Option<usize>,
    ) -> RegisterEntry<'cx>;
}

impl<'cx> Rcc for PeripheralEntry<'cx> {
    fn enr_from_register<'ncx>(
        &'ncx mut self,
        mut register: Register,
        index: Option<usize>,
    ) -> RegisterEntry<'ncx> {
        let name = register.module_name();
        register.ident = format_ident!("{name}enr");

        let title = "peripheral clock enable register";

        self.add_register(register.docs([if let Some(index) = index {
            format!("{name} {title} {index}.")
        } else {
            format!("{name} {title}.")
        }]))
    }
}

/// Peripheral Clock Enable registers implement this trait.
pub trait Enr {
    /// Add a Peripheral Clock Enable field to this register.
    ///
    /// *Note: The suffix "en" is added to the provided name, and the width is fixed at `1`.*
    fn en(&mut self, name: impl AsRef<str>, position: u8) -> Output {
        self.en_from_field(Field::new(name, position, 1))
    }

    /// Add a Peripheral Clock Enable field to this register given a [`Field`] component.
    ///
    /// *Note: The suffix "en" is added to the provided name, and the width is fixed at `1`.*
    fn en_from_field(&mut self, field: Field) -> Output;
}

impl<'cx> Enr for RegisterEntry<'cx> {
    fn en_from_field(&mut self, mut field: Field) -> Output {
        let name = field.module_name();
        field.ident = format_ident!("{name}en");
        field.width = 1;

        let mut en = self.add_store_field(field.docs([format!("{name} clock enable.")]));

        Output {
            disabled: en
                .add_variant(Variant::new("Disabled", 0).docs(["Peripheral clock is disabled."]))
                .make_entitlement(),
            enabled: en
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
