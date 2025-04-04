#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use g4::common::{exti, rcc, syscfg};

    #[test]
    fn gpio_trigger() {
        let rcc: rcc::Reset = unsafe { core::mem::transmute(()) };
        let syscfg: syscfg::Reset = unsafe { core::mem::transmute(()) };
        let exti: exti::Reset = unsafe { core::mem::transmute(()) };

        let syscfgen = rcc
            .apb2enr
            .build_state()
            .syscfgen()
            .enabled()
            .finish()
            .syscfgen;

        cortex_m::asm::delay(1);

        let syscfg = syscfg
            .attach(syscfgen.into())
            .exticr4(|state| state.exti13().pc());

        let exti = exti
            .imr1(|state| state.im13().unmasked())
            .ftsr1(|state| state.ft13().enabled());
    }
}
