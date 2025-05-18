#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

extern crate napi_build;

fn main() {
    napi_build::setup();

    if std::env::var("CARGO_FEATURE_CONTRACT_THREADING").is_ok() {
        println!("cargo:warning=The contract-threading feature is NOT SAFE AND EXPERIMENTAL and not fully implemented yet.");
    }

    if std::env::var("CARGO_FEATURE_ZK_SNARK").is_ok() {
        println!("cargo:warning=The zk-snark feature is NOT SAFE AND EXPERIMENTAL and not fully implemented yet.");
    }

    if std::env::var("CARGO_FEATURE_VDF").is_ok() {
        println!("cargo:warning=The vdf feature is NOT SAFE AND EXPERIMENTAL and not fully implemented yet.");
    }
}
