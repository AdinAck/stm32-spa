pub mod ccr;
pub mod cmar;
pub mod cndtr;
pub mod cpar;
pub mod ifcr;
pub mod isr;

use proto_hal_build::ir::structures::{entitlement::Entitlement, peripheral::Peripheral};

#[derive(Clone, Copy)]
pub enum Instance {
    Dma1,
    Dma2,
}

impl Instance {
    fn ident(&self) -> String {
        match self {
            Instance::Dma1 => "dma1",
            Instance::Dma2 => "dma2",
        }
        .to_string()
    }

    fn base_addr(&self) -> u32 {
        match self {
            Instance::Dma1 => 0x4002_0000,
            Instance::Dma2 => 0x4002_0400,
        }
    }
}

pub fn generate(instance: Instance, channels: u8) -> Peripheral {
    Peripheral::new(
        instance.ident(),
        instance.base_addr(),
        [isr::generate(), ifcr::generate()]
            .into_iter()
            .chain((0..channels).flat_map(|channel| {
                [
                    ccr::generate(instance, channel),
                    cndtr::generate(instance, channel),
                    cpar::generate(instance, channel),
                    cmar::generate(instance, channel),
                ]
            })),
    )
    .entitlements([Entitlement::to(format!(
        "rcc::ahb1enr::{}en::Enabled",
        instance.ident()
    ))])
}
