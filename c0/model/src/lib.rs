pub mod rcc;

use proto_hal_model::Model;

use crate::rcc::rcc;

// TODO
#[derive(Debug, Default)]
pub struct Configuration {}

impl Configuration {
    pub fn c092() -> Self {
        Self {}
    }
}

pub fn model(_config: Configuration) -> Model {
    let mut model = Model::new();

    rcc(&mut model);

    model
}
