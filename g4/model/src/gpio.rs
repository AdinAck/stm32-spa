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

use proto_hal_build::ir::structures::{entitlement::Entitlement, peripheral::Peripheral};

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
    fn ident(&self) -> String {
        match self {
            Instance::A => "gpioa",
            Instance::B => "gpiob",
            Instance::C => "gpioc",
            Instance::D => "gpiod",
            Instance::E => "gpioe",
            Instance::F => "gpiof",
            Instance::G => "gpiog",
        }
        .to_string()
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

pub fn generate(instance: Instance) -> Peripheral {
    Peripheral::new(
        instance.ident(),
        instance.base_addr(),
        [
            moder::generate(instance),
            otyper::generate(),
            ospeedr::generate(instance),
            pupdr::generate(instance),
            idr::generate(),
            odr::generate(),
            // bsrr::generate(), // TODO: requires "effects"
            // lckr::generate(), // TODO
            afr::generate(afr::Instance::L),
            afr::generate(afr::Instance::H),
            // brr::generate(), // TODO: requires "effects"
        ],
    )
    .entitlements([Entitlement::to(format!(
        "rcc::ahb2enr::{}en::Enabled",
        instance.ident()
    ))])
}
