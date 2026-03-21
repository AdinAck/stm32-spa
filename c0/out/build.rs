use model::{compose, Configuration};

fn main() {
    let variant = if cfg!(feature = "c092") {
        Configuration::c092()
    } else {
        Configuration::default()
    };

    phb::render(&compose(variant));

    // prevent recompiling when tests change
    println!("cargo::rerun-if-changed=../model/src");
    println!("cargo::rerun-if-changed=../../peripherals/src");
}
