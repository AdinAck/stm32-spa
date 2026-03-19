use stm32c0_spa_model::{Configuration, model};

fn main() {
    phm::validate(model(Configuration::c092()));
}
