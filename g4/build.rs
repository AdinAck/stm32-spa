use interrupts::g4::INTERRUPT_IDENTS;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("chip version must be specified")]
    ChipUnspecified,
}

fn main() -> Result<(), BuildError> {
    if cfg!(any(
        feature = "g431",
        feature = "g441",
        feature = "g474",
        feature = "g484"
    )) {
        println!("cargo::rustc-cfg=chip_selected");
    } else {
        Err(BuildError::ChipUnspecified)?
    }

    println!("cargo::rustc-check-cfg=cfg(chip_selected, values(none()))");
    proto_hal_build::interrupts::build(INTERRUPT_IDENTS);

    Ok(())
}
