pub mod enr;

use proto_hal_model::{Model, Peripheral};

use crate::rcc::enr::{
    ahb1::{self, ahb1enr},
    ahb2::{self, ahb2enr},
    apb2::{self, apb2enr},
};

pub struct Output {
    pub ahb1enr: ahb1::Output,
    pub ahb2enr: ahb2::Output,
    pub apb2enr: apb2::Output,
}

pub fn rcc(model: &mut Model) -> Output {
    let mut rcc = model.add_peripheral(Peripheral::new("rcc", 0x4002_1000));

    Output {
        ahb1enr: ahb1enr(&mut rcc),
        ahb2enr: ahb2enr(&mut rcc),
        apb2enr: apb2enr(&mut rcc),
    }
}
