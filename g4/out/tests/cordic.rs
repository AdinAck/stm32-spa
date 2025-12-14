#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::{assert, assert_eq};
    use fixed::types::I1F31;
    use stm32g4_spa as hal;

    use hal::{cordic, rcc};

    #[test]
    fn sqrt() {
        let p = unsafe { hal::assume_reset() };

        let cordicen = hal::modify! {
            rcc::ahb1enr::cordicen(p.rcc.ahb1enr.cordicen) => Enabled,
        };

        let mut cordic = hal::unmask! {
            cordic(p.cordic),
            rcc::ahb1enr::cordicen(cordicen),
        };

        hal::modify! {
            cordic::csr {
                func(cordic.csr.func) => Sqrt,
                scale(&cordic.csr.scale),
            }
        };

        cortex_m::asm::delay(2);

        assert!(
            hal::read!(cordic::csr::rrdy(&mut cordic.csr.rrdy)).is_no_data(),
            "expected data to not be ready before use"
        );

        let (mut arg, mut res) = hal::unmask! {
            cordic {
                wdata::arg(cordic.wdata.arg),
                rdata::res(cordic.rdata.res),
                csr {
                    argsize(cordic.csr.argsize),
                    ressize(cordic.csr.ressize),
                }
            }
        };

        hal::write! {
            cordic::wdata::arg(&mut arg) => I1F31::from_num(0.25).to_bits() as u32,
        };

        assert_eq!(
            I1F31::from_bits(hal::read!(cordic::rdata::res(&mut res)) as _).to_num::<f32>(),
            0.4999994,
            "expected sqrt(0.25) to be roughly 0.5"
        );
    }
}
