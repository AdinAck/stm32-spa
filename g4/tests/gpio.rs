#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert;
    use g4::common::{gpio::gpioa, rcc};

    #[test]
    fn output_input() {
        let rcc: rcc::Reset = unsafe { core::mem::transmute(()) };
        let gpioa: gpioa::Reset = unsafe { core::mem::transmute(()) };

        let gpioaen = rcc
            .ahb2enr
            .build_state()
            .gpioaen()
            .enabled()
            .finish()
            .gpioaen;

        let gpioa = gpioa
            .attach(gpioaen.into())
            .moder(|state| state.mode5().output());

        let gpioa = gpioa.odr(|state| state.od5().high());

        cortex_m::asm::delay(1);

        assert!(gpioa.idr.read(|r| r.id5()));
        assert!(!gpioa.idr.read(|r| r.id4()));
    }
}
