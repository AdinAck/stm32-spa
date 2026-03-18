use stm32g4_spa_model::{Configuration, model};

fn main() -> phm::Result<()> {
    for variant in [
        Configuration::g431(),
        Configuration::g441(),
        Configuration::g474(),
        Configuration::g484(),
    ] {
        println!("=== Variant: {variant:?} ===");
        phm::validate(&model(variant)?);
    }

    Ok(())
}
