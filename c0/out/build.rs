use model::{Configuration, model};

fn main() -> Result<(), String> {
    let variant = if cfg!(feature = "c092") {
        Configuration::c092()
    } else {
        Configuration::default()
    };

    proto_hal_build::render(&model(variant));

    // prevent recompiling when tests change
    println!("cargo::rerun-if-changed=../model");

    Ok(())
}
