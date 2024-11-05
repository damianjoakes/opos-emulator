use crate::core::config::get_config;
use crate::core::interpretables::built_in::{EscInterpreter, GsInterpreter};
use crate::core::interpretables::ByteInterpreter;
use crate::core::serial::SerialChannel;
use crate::interop::opos::{ESC, EXCLAMATION, GS, LOWER_R, LOWER_T, MINUS, OPEN_CURLY, SP, UPPER_E, UPPER_G, UPPER_M, UPPER_R, UPPER_V};

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

#[test]
fn test_interpretables() {
    let buffer = [
        ESC, SP, 44, GS, 98, 132, 77, 89, 0,
        ESC, EXCLAMATION, 32, 0,
        ESC, MINUS, 255, 0,
        ESC, UPPER_E, 231, 0,
        ESC, UPPER_G, 90, 0,
        ESC, UPPER_M, 81, 0,
        ESC, UPPER_R, 1, 0,
        ESC, UPPER_V, 12, 0,
        ESC, LOWER_R, 56, 0,
        ESC, LOWER_T, 77, 0,
        ESC, OPEN_CURLY, 178, 0,
    ];
    let mut interpreter = ByteInterpreter::new(&buffer);

    let esc_handler = EscInterpreter::new();
    let gs_handler = GsInterpreter::new();

    interpreter.add_interpretable(ESC, Box::new(esc_handler));
    interpreter.add_interpretable(GS, Box::new(gs_handler));

    let result = interpreter.interpret_buffer().unwrap();
    dbg!(result);
}