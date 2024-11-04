use std::error::Error;
use std::fmt::{Debug};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::time::Duration;

use serde::{Deserialize};
use serialport::{DataBits, FlowControl, Parity, StopBits};

/// The raw TOML interpretation of the finalized Serial configuration struct. This is a wrapper
/// around the Serial information that's been parsed out of the TOML config file, which will be
/// converted into a `SerialConfiguration` struct.
#[derive(Debug, Deserialize)]
struct RawSerialConfiguration {
    baud_rate: u32,
    bit_length: u8,
    flow_control: String,
    parity: String,
    timeout: usize,
    stop_bits: String,
    input_path: String,
    output_path: String,
}

/// The raw TOML interpretation of the finalized configuration struct. This is a wrapper
/// around the configuration that's been parsed out of the TOML config file, which will be
/// converted into a `Configuration` struct.
#[derive(Debug, Deserialize)]
struct RawConfiguration {
    serial: RawSerialConfiguration,
}

/// The deserialized serial configuration.
#[derive(Debug)]
pub struct SerialConfiguration {
    pub baud_rate: u32,
    pub bit_length: DataBits,
    pub flow_control: FlowControl,
    pub parity: Parity,
    pub timeout: Duration,
    pub stop_bits: StopBits,
    pub input_path: String,
    pub output_path: String,
}

/// The deserialized configuration object.
#[derive(Debug)]
pub struct Configuration {
    pub serial: SerialConfiguration,
}

impl From<RawConfiguration> for Configuration {
    fn from(value: RawConfiguration) -> Configuration {
        let baud_rate: u32 = value.serial.baud_rate;
        let bit_length = match value.serial.bit_length {
            5 => DataBits::Five,
            6 => DataBits::Six,
            7 => DataBits::Seven,
            8 => DataBits::Eight,
            _default => DataBits::Eight
        };
        let flow_control = match value.serial.flow_control.as_str() {
            "Software" => FlowControl::Software,
            "None" => FlowControl::None,
            "Hardware" => FlowControl::Hardware,
            _default => FlowControl::Hardware
        };
        let parity = match value.serial.parity.as_str() {
            "None" => Parity::None,
            "Odd" => Parity::Odd,
            "Even" => Parity::Even,
            _default => Parity::None
        };
        let timeout = Duration::from_secs(value.serial.timeout as u64);
        let stop_bits = match value.serial.stop_bits.as_str() {
            "One" => StopBits::One,
            "Two" => StopBits::Two,
            _default => StopBits::One
        };
        let input_path = value.serial.input_path;
        let output_path = value.serial.output_path;

        Self {
            serial: SerialConfiguration {
                baud_rate,
                bit_length,
                flow_control,
                parity,
                timeout,
                stop_bits,
                input_path,
                output_path,
            },
        }
    }
}

pub fn get_config<P: AsRef<Path>>(path: P) -> Result<Configuration, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();

    let _ = reader.read_to_string(&mut data)?;

    let config: RawConfiguration = toml::from_str(data.as_str())?;

    Ok(config.into())
}