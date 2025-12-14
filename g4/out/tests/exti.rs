#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_spa as core_hal;
use stm32g4_spa as hal;

use core_hal::nvic;
use hal::{exti, gpioa, interrupt, rcc};

static mut FLAG: bool = false;

#[interrupt]
fn EXTI9_5() {
    unsafe { FLAG = true };

    assert!(
        unsafe { core_hal::read_untracked!(nvic::iabr1::active23).is_active() },
        "interrupt is currently active but not reported as such"
    );

    unsafe {
        hal::write_from_zero_untracked!(exti::pr1::pif5 => Clear);
    };
}

#[defmt_test::tests]
mod tests {
    use super::*;
    use defmt::assert;

    #[before_each]
    fn reset() {
        unsafe {
            hal::write_from_zero_untracked! {
                exti::pr1::pif5 => Clear,
            };

            hal::write_from_reset_untracked! {
                exti {
                    imr1,
                    rtsr1,
                },
                gpioa {
                    odr,
                    moder,
                },
                rcc::ahb2enr,
            };
        }

        cortex_m::asm::delay(2);
    }

    #[test]
    fn gpio_trigger() {
        let p = unsafe { hal::assume_reset() };

        let gpioaen = hal::modify! {
            rcc::ahb2enr::gpioaen(p.rcc.ahb2enr.gpioaen) => Enabled,
        };

        cortex_m::asm::delay(2);

        let mut gpioa = hal::unmask! {
            gpioa(p.gpioa),
            rcc::ahb2enr::gpioaen(gpioaen),
        };

        let mut exti = p.exti;

        hal::modify! {
            gpioa::moder::mode5(gpioa.moder.mode5) => Output,
            exti {
                imr1::im5(exti.imr1.im5) => Unmasked,
                rtsr1::rt5(exti.rtsr1.rt5) => Enabled,
            }
        };

        cortex_m::asm::delay(2);

        assert!(
            hal::read!(exti::pr1::pif5(&mut exti.pr1.pif5)).is_idle(),
            "expected exti interrupt state to start idle"
        );

        hal::modify! {
            gpioa::odr::od5(gpioa.odr.od5) => High,
        };

        cortex_m::asm::delay(2);

        assert!(
            hal::read!(gpioa::idr::id5(&mut gpioa.idr.id5)).is_high(),
            "expected gpio pin level to be high"
        );
        assert!(
            hal::read!(exti::pr1::pif5(&mut exti.pr1.pif5)).is_pending(),
            "expected exti interrupt to be pending"
        );
    }

    #[test]
    fn gpio_trigger_with_nvic() {
        let cp = unsafe { core_hal::assume_reset() };
        let p = unsafe { hal::assume_reset() };

        let mut nvic = cp.nvic;

        unsafe {
            core_hal::write! {
                nvic::iser1::setena23(&mut nvic.iser1.setena23) => Enable,
            }
        };

        let gpioaen = hal::modify! {
            rcc::ahb2enr::gpioaen(p.rcc.ahb2enr.gpioaen) => Enabled,
        };

        cortex_m::asm::delay(2);

        let gpioa = hal::unmask! {
            gpioa(p.gpioa),
            rcc::ahb2enr::gpioaen(gpioaen),
        };

        let mut exti = p.exti;

        hal::modify! {
            gpioa::moder::mode5(gpioa.moder.mode5) => Output,
            exti {
                imr1::im5(exti.imr1.im5) => Unmasked,
                rtsr1::rt5(exti.rtsr1.rt5) => Enabled,
            }
        };

        cortex_m::asm::delay(2);

        assert!(
            hal::read!(exti::pr1::pif5(&mut exti.pr1.pif5)).is_idle(),
            "expected exti interrupt state to start idle"
        );

        hal::modify! {
            gpioa::odr::od5(gpioa.odr.od5) => High,
        };

        cortex_m::asm::delay(100); // padding, just in case

        assert!(unsafe { FLAG }, "interrupt failed to update flag");
    }
}
