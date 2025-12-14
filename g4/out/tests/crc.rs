#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert_eq;
    use stm32g4_spa as hal;

    use hal::{crc, rcc};

    #[test]
    fn initial() {
        let p = unsafe { hal::assume_reset() };

        let crcen = hal::modify! {
            rcc::ahb1enr::crcen(p.rcc.ahb1enr.crcen) => Enabled,
        };

        cortex_m::asm::delay(2);

        let mut crc = hal::unmask! {
            crc(p.crc),
            rcc::ahb1enr::crcen(crcen),
        };

        assert_eq!(
            hal::read!(crc::dr::dr(&mut crc.dr.dr)),
            crc.init.init.value()
        );
    }
}
