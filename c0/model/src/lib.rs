pub mod rcc;

use phm::Model;

use crate::rcc::rcc;

// TODO
#[derive(Debug, Default)]
pub struct Configuration {}

impl Configuration {
    pub fn c092() -> Self {
        Self {}
    }
}

pub fn model(_config: Configuration) -> phm::Result<Model> {
    let mut model = Model::new();

    rcc(&mut model);

    Ok(model)
}
