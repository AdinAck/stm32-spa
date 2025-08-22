#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use core::sync::atomic::{Ordering, fence};

    use defmt::{assert_eq, panic, println};
    use fixed::types::I1F31;
    use g4::{cordic, dma1, dmamux, rcc};

    #[test]
    fn basic() {
        const N: usize = 2;
        static DST: [u32; N] = [0; N];

        let p = unsafe { g4::peripherals() };

        critical_section::with(|cs| {
            let rcc::ahb1enr::States {
                cordicen,
                dma1en,
                dmamux1en,
                ..
            } = rcc::ahb1enr::modify_in_cs(cs, |_, w| {
                w.cordicen(p.rcc.ahb1enr.cordicen)
                    .enabled()
                    .dma1en(p.rcc.ahb1enr.dma1en)
                    .enabled()
                    .dmamux1en(p.rcc.ahb1enr.dmamux1en)
                    .enabled()
            });

            cortex_m::asm::delay(2);

            // cordic
            let cordic = p.cordic.unmask(cordicen);

            cordic::csr::modify_in_cs(cs, |_, w| {
                w.func(cordic.csr.func)
                    .sqrt()
                    .scale(cordic.csr.scale)
                    .preserve()
                    .dmaren(cordic.csr.dmaren)
                    .enabled()
            });

            let mut arg = cordic.wdata.arg.unmask(cordic.csr.argsize);

            // dma
            let mut dma1 = p.dma1.unmask(dma1en);

            // configure channel
            let dma1::ccr0::States { psize, msize, .. } = dma1::ccr0::modify_in_cs(cs, |_, w| {
                w.dir(dma1.ccr0.dir, &dma1.ccr0.en)
                    .read_from_peripheral()
                    .minc(dma1.ccr0.minc, &dma1.ccr0.en)
                    .enabled()
                    .psize(dma1.ccr0.psize, &dma1.ccr0.en)
                    .bits_32()
                    .msize(dma1.ccr0.msize, &dma1.ccr0.en)
                    .bits_32()
            });

            let pa32 = dma1.cpar0.pa32.unmask(psize);
            let mut ma32 = dma1.cmar0.ma32.unmask(msize);

            // peripheral address
            dma1::cpar0::write(|w| {
                w.pa32(pa32, &dma1.ccr0.en)
                    .value::<{ (cordic::base_addr() + cordic::rdata::OFFSET) as _ }>()
            });

            let dst_addr = (&raw const DST).addr();

            // memory address
            dma1::cmar0::write(|w| w.ma32(&mut ma32, &dma1.ccr0.en).value(dst_addr as u32));

            // transfer length
            dma1::cndtr0::write(|w| w.ndt(&mut dma1.cndtr0.ndt, N as u32, &dma1.ccr0.en));

            // mux
            let dmamux = p.dmamux.unmask(dmamux1en);

            // assign request source
            dmamux::c0cr::modify_in_cs(cs, |_, w| w.dmareq(dmamux.c0cr.dmareq).cordic_read());

            fence(Ordering::SeqCst);

            // enable transfer
            dma1::ccr0::modify_in_cs(cs, |_, w| w.en(dma1.ccr0.en).enabled());

            fence(Ordering::SeqCst);

            let ccr0_read = unsafe { dma1::ccr0::read_untracked() };

            assert!(ccr0_read.dir().is_read_from_peripheral());
            assert!(ccr0_read.minc().is_enabled());
            assert!(ccr0_read.mem2mem().is_disabled());
            assert!(ccr0_read.en().is_enabled());

            let c0cr_read = unsafe { dmamux::c0cr::read_untracked() };

            assert!(c0cr_read.dmareq().is_cordic_read());

            fence(Ordering::SeqCst);

            for value in [0.25, 0.16] {
                cordic::wdata::write(|w| w.arg(&mut arg, I1F31::from_num(value).to_bits() as u32));
            }

            let mut count = 0;

            loop {
                if dma1::isr::read().tcif1(&mut dma1.isr.tcif1).is_occurred() {
                    break;
                }

                let rgsr_read = unsafe { dma1::isr::read_untracked() };
                assert!(rgsr_read.teif1().is_no_event());

                if count > 100 {
                    println!(
                        "{}",
                        unsafe { cordic::csr::read_untracked() }.rrdy().is_ready()
                    );
                    panic!("DMA transfer stalled.");
                }

                count += 1;
                cortex_m::asm::delay(100_000);
            }

            fence(Ordering::SeqCst);

            assert_eq!(I1F31::from_bits(DST[0] as _).to_num::<f32>(), 0.5);
            assert_eq!(I1F31::from_bits(DST[1] as _).to_num::<f32>(), 0.4);
        });
    }
}
