pub mod ahb1enr;
pub mod ahb2enr;
pub mod apb2enr;

use proto_hal_model::{Model, Peripheral};

use crate::rcc::{ahb1enr::ahb1enr, ahb2enr::ahb2enr, apb2enr::apb2enr};

pub struct Output {
    pub ahb1enr: ahb1enr::Output,
    pub ahb2enr: ahb2enr::Output,
    pub apb2enr: apb2enr::Output,
}

pub fn rcc(model: &mut Model) -> Output {
    let mut rcc = model.add_peripheral(Peripheral::new("rcc", 0x4002_1000));

    Output {
        ahb1enr: ahb1enr(&mut rcc),
        ahb2enr: ahb2enr(&mut rcc),
        apb2enr: apb2enr(&mut rcc),
    }
}
