pub mod rcc;

use phm::ModelBuilder;

use crate::rcc::rcc;

// TODO
#[derive(Debug, Default)]
pub struct Configuration {}

impl Configuration {
    pub fn c092() -> Self {
        Self {}
    }
}

pub fn model(_config: Configuration) -> ModelBuilder {
    let mut model = ModelBuilder::new();

    rcc(&mut model);

    model
}
