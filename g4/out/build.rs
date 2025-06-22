use quote::ToTokens as _;
use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hal.rs");

    match model::generate() {
        Ok(hal) => {
            fs::write(&dest_path, hal.to_token_stream().to_string()).unwrap();
        }
        Err(_diagnostics) => {
            println!("cargo::error=HAL generation failed. Refer to the model crate for details.");
        }
    }
}
