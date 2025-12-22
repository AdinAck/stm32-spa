use proto_hal_model::{Entitlement, Field, Register, model::PeripheralEntry};

use crate::rcc::enr::add_field;

pub struct Output {
    pub dma1en: Entitlement,
    pub dma2en: Entitlement,
    pub dmamux1en: Entitlement,
    pub cordicen: Entitlement,
    pub fmacen: Entitlement,
    pub flashen: Entitlement,
    pub crcen: Entitlement,
}

pub fn ahb1enr<'cx>(rcc: &mut PeripheralEntry<'cx>) -> Output {
    let mut ahb1enr = rcc.add_register(Register::new("ahb1enr", 0x48).reset(0x100));

    Output {
        dma1en: add_field(&mut ahb1enr, Field::new("dma1en", 0, 1)),
        dma2en: add_field(&mut ahb1enr, Field::new("dma2en", 1, 1)),
        dmamux1en: add_field(&mut ahb1enr, Field::new("dmamux1en", 2, 1)),
        cordicen: add_field(&mut ahb1enr, Field::new("cordicen", 3, 1)),
        fmacen: add_field(&mut ahb1enr, Field::new("fmacen", 4, 1)),
        flashen: add_field(&mut ahb1enr, Field::new("flashen", 8, 1)),
        crcen: add_field(&mut ahb1enr, Field::new("crcen", 12, 1)),
    }
}
