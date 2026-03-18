use model::{Configuration, model};

fn main() -> Result<(), String> {
    let variant = if cfg!(feature = "c092") {
        Configuration::c092()
    } else {
        Configuration::default()
    };

    phb::render(&model(variant).map_err(|e| format!("{e:?}"))?);

    // prevent recompiling when tests change
    println!("cargo::rerun-if-changed=../model");

    Ok(())
}
