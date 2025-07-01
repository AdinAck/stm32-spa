#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_spa::nvic;
use g4::interrupt;
use g4::{exti, gpioa, rcc};

static mut FLAG: bool = false;

#[interrupt]
fn EXTI9_5() {
    unsafe { FLAG = true };

    assert!(
        nvic::iabr1::read().active23().is_active(),
        "interrupt is currently active but not reported as such"
    );

    exti::pr1::write(|w| w.pif5().clear());
}

#[defmt_test::tests]
mod tests {
    use super::*;
    use defmt::assert;

    #[before_each]
    fn reset() {
        unsafe {
            exti::pr1::write(|w| w.pif5().clear());
            exti::imr1::write_from_reset_untracked(|w| w);
            exti::rtsr1::write_from_reset_untracked(|w| w);

            gpioa::odr::write_from_reset_untracked(|w| w);
            gpioa::moder::write_from_reset_untracked(|w| w);

            rcc::ahb2enr::write_from_reset_untracked(|w| w);
        }

        cortex_m::asm::delay(2);
    }

    #[test]
    fn gpio_trigger() {
        let p = unsafe { g4::peripherals() };

        let rcc::ahb2enr::States { gpioaen, .. } =
            rcc::ahb2enr::transition(|reg| reg.gpioaen(p.rcc.ahb2enr.gpioaen).enabled());

        cortex_m::asm::delay(2);

        let gpioa = p.gpioa.unmask(gpioaen);

        gpioa::moder::transition(|reg| reg.mode5(gpioa.moder.mode5).output());

        exti::imr1::transition(|reg| reg.im5(p.exti.imr1.im5).unmasked());
        exti::rtsr1::transition(|reg| reg.rt5(p.exti.rtsr1.rt5).enabled());

        cortex_m::asm::delay(2);

        assert!(
            exti::pr1::read().pif5().is_idle(),
            "expected exti interrupt state to start idle"
        );

        gpioa::odr::transition(|reg| reg.od5(gpioa.odr.od5).high());

        cortex_m::asm::delay(2);

        assert!(
            gpioa::idr::read().id5().is_high(),
            "expected gpio pin level to be high"
        );
        assert!(
            exti::pr1::read().pif5().is_pending(),
            "expected exti interrupt to be pending"
        );
    }

    #[test]
    fn gpio_trigger_with_nvic() {
        let p = unsafe { g4::peripherals() };

        nvic::iser1::write(|w| w.setena23().enable());

        let rcc::ahb2enr::States { gpioaen, .. } =
            rcc::ahb2enr::transition(|reg| reg.gpioaen(p.rcc.ahb2enr.gpioaen).enabled());

        cortex_m::asm::delay(2);

        let gpioa = p.gpioa.unmask(gpioaen);

        gpioa::moder::transition(|reg| reg.mode5(gpioa.moder.mode5).output());

        exti::imr1::transition(|reg| reg.im5(p.exti.imr1.im5).unmasked());
        exti::rtsr1::transition(|reg| reg.rt5(p.exti.rtsr1.rt5).enabled());

        cortex_m::asm::delay(2);

        assert!(
            exti::pr1::read().pif5().is_idle(),
            "expected exti interrupt state to start idle"
        );

        gpioa::odr::transition(|reg| reg.od5(gpioa.odr.od5).high());

        cortex_m::asm::delay(100); // padding, just in case

        assert!(unsafe { FLAG }, "interrupt failed to update flag");
    }
}
