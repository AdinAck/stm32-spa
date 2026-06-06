use std::process::ExitCode;

use enum_iterator::all;
use stm32_spa_model::{Mode, compose};

fn main() -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;

    for variant in all() {
        println!("=== Variant: {variant:?} ===");
        if ExitCode::FAILURE == phm::validate(compose(Some(variant), Mode::Validation)) {
            exit_code = ExitCode::FAILURE
        }
    }

    exit_code
}
