pub mod afr;
pub mod brr;
pub mod bsrr;
pub mod idr;
pub mod lckr;
pub mod moder;
pub mod odr;
pub mod ospeedr;
pub mod otyper;
pub mod pupdr;

use peripherals::rcc::enr;
use proto_hal_model::{Model, Peripheral};

use crate::gpio::{
    afr::afr, idr::idr, moder::moder, odr::odr, ospeedr::ospeedr, otyper::otyper, pupdr::pupdr,
};

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
    fn ident(&self) -> &str {
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

    fn base_addr(&self) -> u32 {
        match self {
            Instance::A => 0x4800_0000,
            Instance::B => 0x4800_0400,
            Instance::C => 0x4800_0800,
            Instance::D => 0x4800_0c00,
            Instance::E => 0x4800_1000,
            Instance::F => 0x4800_1400,
            Instance::G => 0x4800_1800,
        }
    }
}

pub fn gpio(model: &mut Model, instance: Instance, gpioen: enr::Output) {
    let mut gpio = model.add_peripheral(Peripheral::new(instance.ident(), instance.base_addr()));

    gpio.ontological_entitlements([gpioen.enabled]);

    moder(&mut gpio, instance);
    otyper(&mut gpio);
    ospeedr(&mut gpio, instance);
    pupdr(&mut gpio, instance);
    idr(&mut gpio);
    odr(&mut gpio);
    // bsrr // TODO: requires "effects"
    // lckr // TODO
    afr(&mut gpio, afr::Instance::L);
    afr(&mut gpio, afr::Instance::H);
    // brr // TODO: requires "effects"
}
