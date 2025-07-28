#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert_eq;
    use g4::{crc, rcc};

    #[test]
    fn initial() {
        let p = unsafe { g4::peripherals() };

        let rcc::ahb1enr::States { crcen, .. } = critical_section::with(|cs| {
            rcc::ahb1enr::modify(cs, |_, w| w.crcen(p.rcc.ahb1enr.crcen).enabled())
        });

        cortex_m::asm::delay(2);

        let mut crc = p.crc.unmask(crcen);

        assert_eq!(crc::dr::read().dr(&mut crc.dr.dr), crc.init.init.value());
    }
}
