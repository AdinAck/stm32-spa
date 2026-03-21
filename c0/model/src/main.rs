use stm32c0_spa_model::{Configuration, compose};

fn main() {
    phm::validate(compose(Configuration::c092()));
}
