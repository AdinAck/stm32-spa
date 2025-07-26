#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use defmt::unwrap;
use g4::{cordic, rcc};
use probe_plotter::make_metric;

#[entry]
fn main() -> ! {
    let mut cos_plot = unwrap!(make_metric!(COSINE: i16 = 0));
    let mut sin_plot = unwrap!(make_metric!(SINE: i16 = 0));

    let p = unsafe { g4::peripherals() };

    let rcc::ahb1enr::States { cordicen, .. } =
        rcc::ahb1enr::modify(|_, w| w.cordicen(p.rcc.ahb1enr.cordicen).enabled());
    let mut cordic = p.cordic.unmask(cordicen);

    let cordic::csr::States { ressize, .. } =
        cordic::csr::modify(|_, w| w.ressize(cordic.csr.ressize).q15());

    cortex_m::asm::delay(2);

    let mut i = 0;

    loop {
        cordic::wdata::write(|w| w.arg(&mut cordic.wdata.arg, &cordic.csr.argsize, i));

        let cos =
            cordic::rdata::read().res0(&mut cordic.rdata.res0, &ressize, &cordic.csr.nres) as i16;

        let sin =
            cordic::rdata::read().res1(&mut cordic.rdata.res1, &ressize, &cordic.csr.nres) as i16;

        cos_plot.set(cos);
        sin_plot.set(sin);

        i = i.wrapping_add(10_000);
    }
}
