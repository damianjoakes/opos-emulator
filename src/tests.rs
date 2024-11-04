use crate::core::config::get_config;

#[test]
fn test_config() {
    let config = get_config("root_files/config.toml").unwrap();
    let _ = dbg!(config);
}