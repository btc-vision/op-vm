pub mod common;

pub mod integration;

#[cfg(test)]
pub fn init_test_logger() {
    #[cfg(feature = "debug-metering")]
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    #[cfg(feature = "debug-metering")]
    log::set_max_level(log::LevelFilter::Trace);
}
