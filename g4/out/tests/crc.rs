#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use ::crc::Crc;
    use defmt::{assert_eq, println};
    use g4::{crc, rcc};

    #[before_each]
    fn reset() {
        unsafe {
            rcc::ahb1enr::write_from_reset_untracked(|w| w);
        }

        cortex_m::asm::delay(2);
    }

    #[test]
    fn initial() {
        let p = unsafe { g4::peripherals() };

        let rcc::ahb1enr::States { crcen, .. } = critical_section::with(|cs| {
            rcc::ahb1enr::modify(cs, |_, w| w.crcen(p.rcc.ahb1enr.crcen).enabled())
        });

        cortex_m::asm::delay(2);

        let mut crc = p.crc.unmask(crcen);

        // initial crc digest is equal to the default init value
        assert_eq!(crc::dr::read().dr(&mut crc.dr.dr), crc.init.init.value());

        let crc::init::States { init, .. } =
            crc::init::write(|w| w.init(crc.init.init).value::<0xdeadbeef>());

        critical_section::with(|cs| crc::cr::modify(cs, |_, w| w.rst(&mut crc.cr.rst).set()));

        // after a reset, the crc digest is equal to the reconfigured init value
        assert_eq!(crc::dr::read().dr(&mut crc.dr.dr), init.value());
    }

    #[test]
    fn default_polynomial() {
        let p = unsafe { g4::peripherals() };

        let rcc::ahb1enr::States { crcen, .. } = critical_section::with(|cs| {
            rcc::ahb1enr::modify(cs, |_, w| w.crcen(p.rcc.ahb1enr.crcen).enabled())
        });

        cortex_m::asm::delay(2);

        let crc = p.crc.unmask(crcen);

        // check reset polynomial is as expected.
        assert_eq!(
            unsafe { crc::pol::read_untracked() }.pol(),
            crc.pol.pol.value()
        );
    }

    #[test]
    fn digest() {
        let p = unsafe { g4::peripherals() };

        let rcc::ahb1enr::States { crcen, .. } = critical_section::with(|cs| {
            rcc::ahb1enr::modify(cs, |_, w| w.crcen(p.rcc.ahb1enr.crcen).enabled())
        });

        cortex_m::asm::delay(2);

        let mut crc = p.crc.unmask(crcen);

        // const MOCK: ::crc::Algorithm<u32> = ::crc::Algorithm {
        //     width: 32,
        //     poly: 0x04c1_1db7,
        //     init: 0xffff_ffff,
        //     refin: false,
        //     refout: false,
        //     xorout: 0x0000,
        //     check: 0x0000,
        //     residue: 0x0000,
        // };
        let cpu_crc = Crc::<u32>::new(&::crc::CRC_32_MPEG_2);
        let mut digest = cpu_crc.digest();

        for i in 0..1 {
            crc::dr::write(|w| w.dr(&mut crc.dr.dr, i));
            digest.update(&i.to_be_bytes());
        }

        let finalized = digest.finalize();

        println!(
            "0x{:08x}, 0x{:08x}",
            crc::dr::read().dr(&mut crc.dr.dr),
            finalized
        );
        assert_eq!(crc::dr::read().dr(&mut crc.dr.dr), finalized);
    }
}
