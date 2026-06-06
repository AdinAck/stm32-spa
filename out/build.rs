use model::Mode;
#[allow(unused)]
use model::{
    Device, compose,
    devices::{c0, g4},
};

fn main() {
    let device = cfg_select! {
        feature = "c092" => Some(Device::C0(c0::Variant::C092)),
        feature = "g431" => Some(Device::G4(g4::Variant::G431)),
        feature = "g441" => Some(Device::G4(g4::Variant::G441)),
        feature = "g474" => Some(Device::G4(g4::Variant::G474)),
        feature = "g484" => Some(Device::G4(g4::Variant::G484)),
        _ => None,
    };

    phb::render(&compose(device, Mode::Production));

    // prevent recompiling when tests change
    println!("cargo::rerun-if-changed=../model/src");
}
