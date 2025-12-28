pub mod ccr;
pub mod cmar;
pub mod cndtr;
pub mod cpar;
pub mod ifcr;
pub mod isr;

use proto_hal_model::{Entitlement, Model, Peripheral};

use crate::dma::{ccr::ccr, cmar::cmar, cndtr::cndtr, cpar::cpar, ifcr::ifcr, isr::isr};

#[derive(Clone, Copy)]
pub enum Instance {
    Dma1,
    Dma2,
}

impl Instance {
    fn ident(&self) -> &str {
        match self {
            Instance::Dma1 => "dma1",
            Instance::Dma2 => "dma2",
        }
    }

    fn base_addr(&self) -> u32 {
        match self {
            Instance::Dma1 => 0x4002_0000,
            Instance::Dma2 => 0x4002_0400,
        }
    }
}

pub fn dma(model: &mut Model, instance: Instance, channels: u8, dmaen: Entitlement) {
    let mut dma = model.add_peripheral(Peripheral::new(instance.ident(), instance.base_addr()));

    dma.ontological_entitlements([dmaen]);

    isr(&mut dma);
    ifcr(&mut dma);

    for i in 0..channels {
        let ccr = ccr(&mut dma, i);
        cndtr(&mut dma, i, cndtr::Entitlements { en: ccr.en });
        cpar(
            &mut dma,
            i,
            cpar::Entitlements {
                en: ccr.en,
                psize: ccr.psize,
            },
        );
        cmar(
            &mut dma,
            i,
            cmar::Entitlements {
                en: ccr.en,
                msize: ccr.msize,
            },
        );
    }
}
