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
    pub fn apply_to(&self, composition: &mut Composition) {
        match self {
            Self::C0(variant) => {
                cmpm::Device::M0.apply_to(composition);
                variant.apply_to(composition)
            }
            Self::G4(variant) => {
                cmpm::Device::M4.apply_to(composition);
                variant.apply_to(composition)
            }
        }
    }
}

pub fn compose(device: Option<Device>) -> Composition {
    let mut composition = Composition::default();

    if let Some(device) = device {
        device.apply_to(&mut composition);
    } else {
        composition.add_diagnostic(diagnostic::Rank::Error, "A device must be selected.");
    }

    composition
}
