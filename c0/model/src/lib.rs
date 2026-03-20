pub mod rcc;

use phm::Composition;

use crate::rcc::rcc;

// TODO
#[derive(Debug, Default)]
pub struct Configuration {}

impl Configuration {
    pub fn c092() -> Self {
        Self {}
    }
}

pub fn compose(_config: Configuration) -> Composition {
    let mut model = Composition::new();

    rcc(&mut model);

    model
}
