//! GPIO structures and model component helpers.
//!
//! References:
//! - RM0440
//! - RM0490

use model_macros::peripheral;
use phm::Peripheral;

use crate::peripherals::rcc::enr::en;

pub mod afr;
pub mod idr;
pub mod moder;
pub mod odr;
pub mod ospeedr;
pub mod otyper;
pub mod pupdr;

#[derive(Clone, Copy)]
pub enum Instance {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Instance {
    pub fn name(&self) -> &str {
        match self {
            Instance::A => "gpioa",
            Instance::B => "gpiob",
            Instance::C => "gpioc",
            Instance::D => "gpiod",
            Instance::E => "gpioe",
            Instance::F => "gpiof",
            Instance::G => "gpiog",
        }
    }
}

peripheral! {
    /// Device model compositions implement this trait to attach GPIO peripherals.
    Gpio {
        /// Add a GPIO peripheral to this composition.
        gpio(instance: Instance, base_addr: u32, en: en::EnSchema) {
            let mut gpio = self.add_peripheral(Peripheral::new(instance.name(), base_addr));

            gpio.ontological_entitlements([[en.enabled]]);

            gpio
        }
    }
}
