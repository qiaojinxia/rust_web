// logger.rs
use env_logger::Builder;
use log::LevelFilter;
use std::env;

pub fn init() {
    let env = env::var("RUST_LOG").unwrap_or_else(|_| "debug".to_string());
    Builder::new()
        .filter_level(LevelFilter::Info)
        .parse_filters(&env)
        .init();
}
