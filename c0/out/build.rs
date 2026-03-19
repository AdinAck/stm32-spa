use model::{Configuration, model};

fn main() {
    let variant = if cfg!(feature = "c092") {
        Configuration::c092()
    } else {
        Configuration::default()
    };

    phb::render(&model(variant));

    // prevent recompiling when tests change
    println!("cargo::rerun-if-changed=../model");
}
