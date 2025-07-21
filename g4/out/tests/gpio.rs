#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert;
    use g4::{gpioa, rcc};

    #[test]
    fn output_input() {
        let p = unsafe { g4::peripherals() };

        let rcc::ahb2enr::States { gpioaen, .. } =
            rcc::ahb2enr::modify(|_, w| w.gpioaen(p.rcc.ahb2enr.gpioaen).enabled());

        let mut gpioa = p.gpioa.unmask(gpioaen);

        gpioa::moder::modify(|_, w| w.mode5(gpioa.moder.mode5).output());
        gpioa::odr::modify(|_, w| w.od5(gpioa.odr.od5).high());

        cortex_m::asm::delay(2);

        assert!(
            gpioa::idr::read().id5(&mut gpioa.idr.id5).is_high(),
            "expected controlled pin level to be high"
        );
        assert!(
            gpioa::idr::read().id4(&mut gpioa.idr.id4).is_low(),
            "expected non-controlled pin level to be low"
        );
    }
}
