use stm32g4_spa_model::{Configuration, compose};

fn main() {
    for variant in [
        Configuration::g431(),
        Configuration::g441(),
        Configuration::g474(),
        Configuration::g484(),
    ] {
        println!("=== Variant: {variant:?} ===");
        phm::validate(compose(variant));
    }
}
