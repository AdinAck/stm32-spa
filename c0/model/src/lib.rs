use proto_hal_model::Model;

// TODO
#[derive(Debug, Default)]
pub struct Configuration {}

impl Configuration {
    pub fn c092() -> Self {
        Self {}
    }
}

pub fn model(_config: Configuration) -> Model {
    Model::new()
}
