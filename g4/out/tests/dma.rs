#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use core::sync::atomic::{Ordering, fence};

    use defmt::{assert, assert_eq};
    use fixed::types::I1F31;
    use stm32g4_spa as hal;

    use hal::{cordic, dma1, dmamux, rcc};

    #[test]
    fn basic() {
        const N: usize = 2;
        static mut DST: [u32; N] = [0; N];

        let p = unsafe { hal::assume_reset() };

        critical_section::with(|cs| {
            let (cordicen, dma1en, dma1muxen) = hal::modify! {
                rcc::ahb1enr {
                    cordicen(p.rcc.ahb1enr.cordicen) => Enabled,
                    dma1en(p.rcc.ahb1enr.dma1en) => Enabled,
                    dmamux1en(p.rcc.ahb1enr.dmamux1en) => Enabled,
                }
            };

            cortex_m::asm::delay(2);

            let (cordic, mut dma1, dmamux) = hal::unmask! {
                cordic(p.cordic),
                rcc::ahb1enr::cordicen(cordicen),
                dma1(p.dma1),
                rcc::ahb1enr::dma1en(dma1en),
                dmamux(p.dmamux),
                rcc::ahb1enr::dmamux1en(dma1muxen),
            };

            // cordic
            hal::modify! {
                @critical_section(cs),
                cordic::csr {
                    func(cordic.csr.func) => Sqrt,
                    scale(&cordic.csr.scale),
                    dmaren(cordic.csr.dmaren) => Enabled,
                }
            };

            let mut arg = hal::unmask! {
                cordic {
                    wdata::arg(cordic.wdata.arg),
                    csr::argsize(cordic.csr.argsize),
                }
            };

            // dma

            // configure channel
            let (.., psize, msize) = hal::modify! {
                @critical_section(cs),
                dma1::ccr0 {
                    dir(dma1.ccr0.dir) => ReadFromPeripheral,
                    en(&dma1.ccr0.en),
                    minc(dma1.ccr0.minc) => Enabled,
                    psize(dma1.ccr0.psize) => Bits32,
                    msize(dma1.ccr0.msize) => Bits32,
                }
            };

            let (pa32, mut ma32) = hal::unmask! {
                dma1 {
                    cpar0::pa32(dma1.cpar0.pa32),
                    cmar0::ma32(dma1.cmar0.ma32),
                    ccr0 {
                        psize(psize),
                        msize(msize),
                    }
                }
            };

            // peripheral address

            hal::write! {
                dma1 {
                    cpar0::pa32(pa32) => 0x4002_0c08, // cordic::rdata
                    ccr0::en(&dma1.ccr0.en),
                }
            };

            let dst_addr = (&raw const DST).addr() as _;

            // memory address
            hal::write!(
                dma1 {
                    cmar0::ma32(&mut ma32) => dst_addr,
                    ccr0::en(&dma1.ccr0.en),
                }
            );

            // transfer length
            hal::write! {
                dma1 {
                    cndtr0::ndt(dma1.cndtr0.ndt) => { N as u16 },
                    ccr0::en(&dma1.ccr0.en),
                }
            };

            // mux

            // assign request source
            hal::modify! {
                @critical_section(cs),
                dmamux::c0cr::dmareq(dmamux.c0cr.dmareq) => CordicRead,
            };

            fence(Ordering::SeqCst);

            // enable transfer
            // TODO: this should require ndt as well to make both Dynamic
            hal::modify! {
                @critical_section(cs),
                dma1 {
                    ccr0::en(dma1.ccr0.en) => Enabled,
                }
            };

            fence(Ordering::SeqCst);

            let (dma1_read, dmamux_read) = unsafe {
                hal::read_untracked! {
                    dma1::ccr0,
                    dmamux::c0cr
                }
            };

            assert!(dma1_read.ccr0.dir.is_read_from_peripheral());
            assert!(dma1_read.ccr0.minc.is_enabled());
            assert!(dma1_read.ccr0.mem2mem.is_disabled());
            assert!(dma1_read.ccr0.en.is_enabled());
            assert!(dmamux_read.c0cr.dmareq.is_cordic_read());

            let isr = hal::read!(
                dma1::isr {
                    tcif1(&mut dma1.isr.tcif1),
                    htif1(&mut dma1.isr.htif1),
                    teif1(&mut dma1.isr.teif1),
                }
            );

            assert!(isr.htif1.is_no_event());
            assert!(isr.tcif1.is_no_event());
            assert!(isr.teif1.is_no_event());

            for value in [0.25, 0.16] {
                hal::write! {
                    cordic::wdata::arg(&mut arg) => I1F31::from_num(value).to_bits() as u32,
                };
            }

            cortex_m::asm::delay(100);

            let isr = hal::read!(
                dma1::isr {
                    tcif1(&mut dma1.isr.tcif1),
                    htif1(&mut dma1.isr.htif1),
                    teif1(&mut dma1.isr.teif1),
                }
            );

            assert!(isr.htif1.is_occurred());
            assert!(isr.tcif1.is_occurred());
            assert!(isr.teif1.is_no_event());

            fence(Ordering::SeqCst);

            assert_eq!(
                I1F31::from_bits(unsafe { DST[0] } as _).to_num::<f32>(),
                0.4999994
            );
            assert_eq!(
                I1F31::from_bits(unsafe { DST[1] } as _).to_num::<f32>(),
                0.39999902
            );
        });
    }
}
