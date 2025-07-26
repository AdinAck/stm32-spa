#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use cortex_m_spa::nvic;
use g4::interrupt;
use g4::{exti, gpioa, rcc};

#[interrupt]
fn EXTI9_5() {
    unsafe {
        exti::pr1::write_from_zero_untracked(|w| w.pif5(exti::pr1::pif5::WriteVariant::Clear))
    };

    cortex_m::asm::delay(100_000);
}

#[entry]
fn main() -> ! {
    let cp = unsafe { cortex_m_spa::peripherals() };
    let p = unsafe { g4::peripherals() };

    let mut nvic = cp.nvic;

    nvic::iser1::write(|w| w.setena23(&mut nvic.iser1.setena23).enable());

    cortex_m::asm::delay(100_000);

    let rcc::ahb2enr::States { gpioaen, .. } =
        rcc::ahb2enr::modify(|_, w| w.gpioaen(p.rcc.ahb2enr.gpioaen).enabled());

    cortex_m::asm::delay(100_000);

    let gpioa = p.gpioa.unmask(gpioaen);

    gpioa::moder::modify(|_, w| w.mode5(gpioa.moder.mode5).output());

    exti::imr1::modify(|_, w| w.im5(p.exti.imr1.im5).unmasked());
    exti::rtsr1::modify(|_, w| w.rt5(p.exti.rtsr1.rt5).enabled());

    cortex_m::asm::delay(100_000);

    gpioa::odr::modify(|_, w| w.od5(gpioa.odr.od5).high());

    loop {}
}
