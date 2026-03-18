use model::{Configuration, model};

fn main() -> Result<(), String> {
    let variant = if cfg!(feature = "g431") {
        Configuration::g431()
    } else if cfg!(feature = "g441") {
        Configuration::g441()
    } else if cfg!(feature = "g474") {
        Configuration::g474()
    } else if cfg!(feature = "g484") {
        Configuration::g484()
    } else {
        Configuration::default()
    };

    phb::render(&model(variant).map_err(|e| format!("{e:?}"))?);

    // prevent recompiling when tests change
    println!("cargo::rerun-if-changed=../model");

    Ok(())
}
