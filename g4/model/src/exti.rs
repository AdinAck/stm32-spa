pub mod emr;
pub mod ftsr;
pub mod imr;
pub mod pr;
pub mod rtsr;

use proto_hal_model::{Model, Peripheral};

use crate::exti::{emr::emr, ftsr::ftsr, imr::imr, pr::pr, rtsr::rtsr};

pub fn exti(model: &mut Model) {
    let mut exti = model.add_peripheral(Peripheral::new("exti", 0x4001_0400));

    imr(&mut exti, imr::Instance::I1);
    emr(&mut exti, emr::Instance::I1);
    rtsr(&mut exti, rtsr::Instance::I1);
    ftsr(&mut exti, ftsr::Instance::I1);
    pr(&mut exti, pr::Instance::I1);
}
