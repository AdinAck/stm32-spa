pub mod devices;
pub mod peripherals;

use enum_iterator::Sequence;
use phm::{Composition, diagnostic};

use crate::devices::{c0, g4};

#[derive(Debug, Clone, Copy, Sequence)]
pub enum Device {
    C0(c0::Variant),
    G4(g4::Variant),
}

impl Device {
    pub fn apply_to(&self, composition: &mut Composition, mode: Mode) {
        match self {
            Self::C0(variant) => {
                if let Mode::Validation = mode {
                    cmpm::Device::M0.apply_to(composition);
                }

                variant.apply_to(composition)
            }
            Self::G4(variant) => {
                if let Mode::Validation = mode {
                    cmpm::Device::M4.apply_to(composition);
                }

                variant.apply_to(composition)
            }
        }
    }
}

/// Model composition mode.
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    /// Compose the model for validation.
    Validation,
    /// Compose the model for production.
    Production,
}

pub fn compose(device: Option<Device>, mode: Mode) -> Composition {
    let mut composition = Composition::default();

    if let Some(device) = device {
        device.apply_to(&mut composition, mode);
    } else {
        composition.add_diagnostic(diagnostic::Rank::Error, "A device must be selected.");
    }

    composition
}
