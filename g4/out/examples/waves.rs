#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use defmt::unwrap;
use g4::{cordic, rcc};
use probe_plotter::make_metric;
use proto_hal::stasis::Freeze as _;

#[entry]
fn main() -> ! {
    let mut cos_plot = unwrap!(make_metric!(COSINE: i16 = 0));
    let mut sin_plot = unwrap!(make_metric!(SINE: i16 = 0));

    let p = unsafe { g4::peripherals() };

    let rcc::ahb1enr::States { cordicen, .. } = critical_section::with(|cs| {
        rcc::ahb1enr::modify(cs, |_, w| w.cordicen(p.rcc.ahb1enr.cordicen).enabled())
    });
    let cordic = p.cordic.unmask(cordicen);

    let cordic::csr::States { ressize, .. } = critical_section::with(|cs| {
        cordic::csr::modify(cs, |_, w| w.ressize(cordic.csr.ressize).q15())
    });

    cortex_m::asm::delay(2);

    let mut arg = cordic.wdata.arg.unmask(cordic.csr.argsize);
    let (_, [res0_ressize_ent, res1_ressize_ent]) = ressize.freeze();
    let (_, [res0_nres_ent, res1_nres_ent]) = cordic.csr.nres.freeze();
    let (mut res0, mut res1) = (
        cordic.rdata.res0.unmask(res0_nres_ent, res0_ressize_ent),
        cordic.rdata.res1.unmask(res1_nres_ent, res1_ressize_ent),
    );

    let mut i = 0;

    loop {
        cordic::wdata::write(|w| w.arg(&mut arg, i));

        let cos = cordic::rdata::read().res0(&mut res0) as i16;

        let sin = cordic::rdata::read().res1(&mut res1) as i16;

        cos_plot.set(cos);
        sin_plot.set(sin);

        i = i.wrapping_add(10_000);
    }
}
