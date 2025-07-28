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
        unsafe { nvic::iabr1::read_untracked().active23().is_active() },
        "interrupt is currently active but not reported as such"
    );

    unsafe {
        exti::pr1::write_from_zero_untracked(|w| w.pif5(exti::pr1::pif5::WriteVariant::Clear))
    };
}

#[defmt_test::tests]
mod tests {
    use super::*;
    use defmt::assert;

    #[before_each]
    fn reset() {
        unsafe {
            exti::pr1::write_from_zero_untracked(|w| w.pif5(exti::pr1::pif5::WriteVariant::Clear));
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

        let rcc::ahb2enr::States { gpioaen, .. } = critical_section::with(|cs| {
            rcc::ahb2enr::modify(cs, |_, w| w.gpioaen(p.rcc.ahb2enr.gpioaen).enabled())
        });

        cortex_m::asm::delay(2);

        let mut gpioa = p.gpioa.unmask(gpioaen);
        let mut exti = p.exti;

        critical_section::with(|cs| {
            gpioa::moder::modify(cs, |_, w| w.mode5(gpioa.moder.mode5).output());
            exti::imr1::modify(cs, |_, w| w.im5(exti.imr1.im5).unmasked());
            exti::rtsr1::modify(cs, |_, w| w.rt5(exti.rtsr1.rt5).enabled());
        });

        cortex_m::asm::delay(2);

        assert!(
            exti::pr1::read().pif5(&mut exti.pr1.pif5).is_idle(),
            "expected exti interrupt state to start idle"
        );

        critical_section::with(|cs| gpioa::odr::modify(cs, |_, w| w.od5(gpioa.odr.od5).high()));

        cortex_m::asm::delay(2);

        assert!(
            gpioa::idr::read().id5(&mut gpioa.idr.id5).is_high(),
            "expected gpio pin level to be high"
        );
        assert!(
            exti::pr1::read().pif5(&mut exti.pr1.pif5).is_pending(),
            "expected exti interrupt to be pending"
        );
    }

    #[test]
    fn gpio_trigger_with_nvic() {
        let cp = unsafe { cortex_m_spa::peripherals() };
        let p = unsafe { g4::peripherals() };

        let mut nvic = cp.nvic;

        nvic::iser1::write(|w| w.setena23(&mut nvic.iser1.setena23).enable());

        let rcc::ahb2enr::States { gpioaen, .. } = critical_section::with(|cs| {
            rcc::ahb2enr::modify(cs, |_, w| w.gpioaen(p.rcc.ahb2enr.gpioaen).enabled())
        });

        cortex_m::asm::delay(2);

        let gpioa = p.gpioa.unmask(gpioaen);
        let mut exti = p.exti;

        critical_section::with(|cs| {
            gpioa::moder::modify(cs, |_, w| w.mode5(gpioa.moder.mode5).output());
            exti::imr1::modify(cs, |_, w| w.im5(exti.imr1.im5).unmasked());
            exti::rtsr1::modify(cs, |_, w| w.rt5(exti.rtsr1.rt5).enabled());
        });

        cortex_m::asm::delay(2);

        assert!(
            exti::pr1::read().pif5(&mut exti.pr1.pif5).is_idle(),
            "expected exti interrupt state to start idle"
        );

        critical_section::with(|cs| {
            gpioa::odr::modify(cs, |_, w| w.od5(gpioa.odr.od5).high());
        });

        cortex_m::asm::delay(100); // padding, just in case

        assert!(unsafe { FLAG }, "interrupt failed to update flag");
    }
}
