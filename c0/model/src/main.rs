use stm32c0_spa_model::{Configuration, model};

fn main() {
    proto_hal_model::validate(&model(Configuration::c092()));
}
