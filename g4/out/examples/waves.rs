#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use defmt::unwrap;
use probe_plotter::make_metric;
use stm32g4_spa::{self as hal, cordic};

use hal::rcc;

#[entry]
fn main() -> ! {
    let mut cos_plot = unwrap!(make_metric!(COSINE: i16 = 0));
    let mut sin_plot = unwrap!(make_metric!(SINE: i16 = 0));

    let p = unsafe { hal::assume_reset() };

    let cordicen = hal::modify! {
        rcc::ahb1enr::cordicen(p.rcc.ahb1enr.cordicen) => Enabled,
    };

    let cordic = hal::unmask! {
        cordic(p.cordic),
        rcc::ahb1enr::cordicen(cordicen),
    };

    let ressize = hal::modify! {
        cordic::csr::ressize(cordic.csr.ressize) => Q15,
    };

    cortex_m::asm::delay(2);

    let mut arg = hal::unmask! {
        cordic::wdata::arg(cordic.wdata.arg),
        cordic::csr::argsize(cordic.csr.argsize),
    };

    let (mut res0, mut res1) = hal::unmask! {
        cordic {
            rdata {
                res0(cordic.rdata.res0),
                res1(cordic.rdata.res1),
            },
            csr {
                ressize(ressize),
                nres(cordic.csr.nres)
            }
        }
    };

    let mut i = 0;

    loop {
        hal::write! {
            cordic::wdata::arg(&mut arg) => i,
        }

        let rdata = hal::read! {
            cordic::rdata {
                res0(&mut res0),
                res1(&mut res1),
            }
        };

        cos_plot.set(rdata.res0 as i16);
        sin_plot.set(rdata.res1 as i16);

        i = i.wrapping_add(10_000);
    }
}
