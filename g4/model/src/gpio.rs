use std::array;

use peripherals::{
    gpio::{afr, moder, odr, ospeedr, otyper, pupdr},
    prelude::*,
    rcc::enr::en,
};
use phm::Composition;

pub use peripherals::gpio::Instance;

#[derive(Clone, Copy)]
pub struct Output {
    pub moder: moder::Output,
    pub otyper: otyper::Output,
    pub ospeedr: ospeedr::OutputWithVeryHigh,
    pub pupdr: pupdr::Output,
    pub odr: odr::Output,
    pub afrl: afr::Output,
    pub afrh: afr::Output,
}

pub fn gpio(composition: &mut Composition, instance: Instance, gpioen: en::Output) -> Output {
    let mut gpio = composition.gpio(
        instance,
        match instance {
            Instance::A => 0x4800_0000,
            Instance::B => 0x4800_0400,
            Instance::C => 0x4800_0800,
            Instance::D => 0x4800_0c00,
            Instance::E => 0x4800_1000,
            Instance::F => 0x4800_1400,
            Instance::G => 0x4800_1800,
        },
        gpioen,
    );

    // moder
    let mut moder = gpio
        .moder(
            0,
            match instance {
                Instance::A => 0xabff_ffff,
                Instance::B => 0xffff_febf,
                _ => 0xffff_ffff,
            },
        )
        .docs([
            "*Note: It is recommended to set PB8 to a different mode than the \
        analog one to limit the consumption that would occur if the pin is \
        left unconnected.*",
        ]);
    let moder = array::from_fn(|position| moder.mode(position as _).mode());

    // otyper
    let mut otyper = gpio.otyper(4, 0);
    let otyper = array::from_fn(|position| otyper.ot(position as _).ot());

    // ospeedr
    let mut ospeedr = gpio.ospeedr(
        8,
        match instance {
            Instance::A => 0x0c00_0000,
            _ => 0,
        },
    );
    let ospeedr = array::from_fn(|position| ospeedr.ospeed(position as _).ospeed_with_very_high());

    // pupdr
    let mut pupdr = gpio.pupdr(
        0xc,
        match instance {
            Instance::A => 0x6400_0000,
            Instance::B => 0x100,
            _ => 0,
        },
    );
    let pupdr = array::from_fn(|position| pupdr.pupd(position as _).pupd());

    // idr
    let mut idr = gpio.idr(0x10);
    for position in 0..16 {
        idr.id(position).id();
    }

    // odr
    let mut odr = gpio.odr(0x14);
    let odr = array::from_fn(|position| odr.od(position as _).od());

    // bsrr // TODO: requires "effects"
    // lckr // TODO

    // afrl
    let mut afrl = gpio.afrl(0x20);
    let afrl = array::from_fn(|position| afrl.afsel(position as _).afsel());

    // afrh
    let mut afrh = gpio.afrh(0x24);
    let afrh = array::from_fn(|i| afrh.afsel((i + 8) as _).afsel());

    // brr // TODO: requires "effects"

    Output {
        moder,
        otyper,
        ospeedr,
        pupdr,
        odr,
        afrl,
        afrh,
    }
}
