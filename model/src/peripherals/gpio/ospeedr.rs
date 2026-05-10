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

use phm::{
    Field, Register,
    field::access,
    model::{FieldEntry, PeripheralEntry, RegisterEntry},
};

/// GPIO peripherals implement this trait to attach Port Output Speed registers.
pub trait Ospeedr {
    /// Add a Port Output Speed register to this peripheral.
    fn ospeedr<'ncx>(&'ncx mut self, offset: u32, reset: u32) -> RegisterEntry<'ncx>;
}

impl<'cx> Ospeedr for PeripheralEntry<'cx> {
    fn ospeedr<'ncx>(&'ncx mut self, offset: u32, reset: u32) -> RegisterEntry<'ncx> {
        self.add_register(Register::new("ospeedr", offset).reset(reset))
    }
}

/// Port Output Speed registers implement this trait to attach Output Speed fields.
pub trait Ospeed {
    /// Add an Output Speed field to this register.
    fn ospeed<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store>;
}

impl<'cx> Ospeed for RegisterEntry<'cx> {
    fn ospeed<'ncx>(&'ncx mut self, position: u8) -> FieldEntry<'ncx, access::Store> {
        self.add_store_field(Field::new(format!("ospeed{position}"), position * 2, 2))
    }
}

// Note: Heterogenous registers should be expressed ad hoc and these are purely for convenience.
pub type Output = [ospeed::Output; 16];
pub type OutputWithVeryHigh = [ospeed::OutputWithVeryHigh; 16];

pub mod ospeed {
    use phm::{Entitlement, Variant, field::access, model::FieldEntry};

    /// Port Output Speed fields implement this trait to be appropriately populated.
    pub trait Ospeed {
        /// Populate the Port Output Speed field with the appropriate contents.
        fn ospeed_with_very_high(&mut self) -> OutputWithVeryHigh;

        /// Populate the Port Output Speed field with the appropriate contents.
        ///
        /// This method *omits* the "Very High" speed variant according to RM0490 § 8.5.3.
        fn ospeed(&mut self) -> Output;
    }

    impl<'cx> Ospeed for FieldEntry<'cx, access::Store> {
        fn ospeed_with_very_high(&mut self) -> OutputWithVeryHigh {
            let output = self.ospeed();

            OutputWithVeryHigh {
                low: output.low,
                medium: output.medium,
                high: output.high,
                veryhigh: self
                    .add_variant(Variant::new("VeryHigh", 3))
                    .make_entitlement(),
            }
        }

        fn ospeed(&mut self) -> Output {
            Output {
                low: self.add_variant(Variant::new("Low", 0)).make_entitlement(),
                medium: self
                    .add_variant(Variant::new("Medium", 1))
                    .make_entitlement(),
                high: self.add_variant(Variant::new("High", 2)).make_entitlement(),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct OutputWithVeryHigh {
        pub low: Entitlement,
        pub medium: Entitlement,
        pub high: Entitlement,
        pub veryhigh: Entitlement,
    }

    /// The state outputs of [`Ospeed`] fields *except for* the "Very High" state.
    #[derive(Clone, Copy)]
    pub struct Output {
        pub low: Entitlement,
        pub medium: Entitlement,
        pub high: Entitlement,
    }
}
