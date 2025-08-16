#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::{assert, assert_eq};
    use fixed::types::I1F31;
    use g4::{cordic, rcc};

    #[test]
    fn sqrt() {
        let p = unsafe { g4::peripherals() };

        let rcc::ahb1enr::States { cordicen, .. } =
            rcc::ahb1enr::modify(|_, w| w.cordicen(p.rcc.ahb1enr.cordicen).enabled());
        let mut cordic = p.cordic.unmask(cordicen);

        cordic::csr::modify(|_, w| {
            w.func(cordic.csr.func)
                .sqrt()
                .scale(cordic.csr.scale)
                .preserve()
        });

        cortex_m::asm::delay(2);

        assert!(
            cordic::csr::read().rrdy(&mut cordic.csr.rrdy).is_no_data(),
            "expected data to noy be ready before use"
        );

        let mut arg = cordic.wdata.arg.unmask(cordic.csr.argsize);
        let mut res = cordic.rdata.res.unmask(cordic.csr.ressize);

        cordic::wdata::write(|w| w.arg(&mut arg, I1F31::from_num(0.25).to_bits() as u32));

        assert_eq!(
            I1F31::from_bits(cordic::rdata::read().res(&mut res) as _).to_num::<f32>(),
            0.4999994,
            "expected sqrt(0.25) to be roughly 0.5"
        );
    }
}
