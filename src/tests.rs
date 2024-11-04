use crate::core::config::get_config;
use crate::core::serial::SerialChannel;

#[test]
fn test_config() {
    let config = get_config("root_files/config.toml").unwrap();
    let _ = dbg!(config);
}

#[test]
fn test_serial_port() {
    let pair = SerialChannel::from_config("root_files/config.toml").unwrap();
    dbg!(pair);
}