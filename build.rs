#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

fn main() {
    if std::env::var("CARGO_FEATURE_CONTRACT_THREADING").is_ok() {
        println!("cargo:warning=The contract-threading feature is NOT SAFE AND EXPERIMENTAL and not fully implemented yet.");
    }
}
