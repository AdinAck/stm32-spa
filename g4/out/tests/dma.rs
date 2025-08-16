#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use core::sync::atomic::{Ordering, fence};

    use defmt::{assert_eq, panic};
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

            fence(Ordering::Release);

            // enable transfer
            dma1::ccr0::modify_in_cs(cs, |_, w| w.en(dma1.ccr0.en).enabled());

            for value in [0.25, 0.16] {
                cordic::wdata::write(|w| w.arg(&mut arg, I1F31::from_num(value).to_bits() as u32));
            }

            let mut count = 0;

            loop {
                if dma1::isr::read().tcif1(&mut dma1.isr.tcif1).is_occurred() {
                    break;
                }

                if count > 100 {
                    panic!("DMA transfer stalled.");
                }

                count += 1;
                cortex_m::asm::delay(100);
            }

            fence(Ordering::Acquire);

            assert_eq!(I1F31::from_bits(DST[0] as _).to_num::<f32>(), 0.5);
            assert_eq!(I1F31::from_bits(DST[1] as _).to_num::<f32>(), 0.4);
        });
    }
}
