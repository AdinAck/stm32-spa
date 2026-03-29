#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert;
    use stm32c0_spa as hal;

    use hal::{gpioa, rcc};

    #[test]
    fn output_input() {
        let p = unsafe { hal::assume_reset() };

        let gpioaen = hal::modify! {
            rcc::iopenr::gpioaen(p.rcc.iopenr.gpioaen) => _,
        };

        let mut gpioa = hal::unmask! {
            gpioa(p.gpioa),
            rcc::iopenr::gpioaen(gpioaen),
        };

        hal::modify! {
            gpioa {
                moder::mode5(gpioa.moder.mode5) => Output,
                odr::od5(gpioa.odr.od5) => High,
            }
        };

        cortex_m::asm::delay(2);

        assert!(
            hal::read!(gpioa::idr::id5(&mut gpioa.idr.id5)).is_high(),
            "expected controlled pin level to be high"
        );
        assert!(
            hal::read!(gpioa::idr::id4(&mut gpioa.idr.id4)).is_low(),
            "expected non-controlled pin level to be low"
        );
    }
}
