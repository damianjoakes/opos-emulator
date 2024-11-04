//! Module containing serial port builders from the configuration file.

use std::error::Error;
use std::io::{Read, Write};
use std::path::Path;
use serialport::{ClearBuffer, SerialPort};
use tokio_serial::{SerialStream};
use crate::core::config::get_config;
use crate::interop::opos::ResultCode::OPOS_SUCCESS;
use crate::interop::opos::ResultCodeExtended;

/// The internal application's read/write stream. This struct contains helpers for managing
/// asynchronous read/write operations to the listening COM device for the application.
#[derive(Debug)]
pub struct SerialChannel {
    /// The read/write stream for the application. This hooks onto an **existing** virtual COM
    /// port pair (in the future we may create a virtual COM driver for the application).
    pub channel: SerialStream,
}

impl SerialChannel {
    /// Creates a new channel from the provided configuration file.
    pub fn from_config<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        // Get the application configuration.
        let config = get_config(path)?;
        let serial_config = &config.serial;

        let mut read_channel = tokio_serial::new(&serial_config.output_path, serial_config.baud_rate);
        read_channel = read_channel
            .timeout(serial_config.timeout)
            .stop_bits(serial_config.stop_bits)
            .parity(serial_config.parity)
            .flow_control(serial_config.flow_control)
            .data_bits(serial_config.bit_length);

        let mut channel = SerialStream::open(&read_channel)?;
        channel.write_data_terminal_ready(true)?;

        Ok(
            Self {
                channel
            }
        )
    }

    /// Conditionally the contents of the serial channel into the provided buffer. Returns a Result
    /// containing either the bytes read or the error encountered.
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let channel = &mut self.channel;
        let res = channel.readable().await;
        let bytes = match res {
            Ok(_) => {
                channel.try_read(buf)?
            }
            Err(err) => {
                // todo: Flag DTR as false, in case we want to implement better error handling.
                return Err(err);
            }
        };

        Ok(bytes)
    }

    /// Writes some data to the COM interface.
    pub async fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        let channel = &mut self.channel;
        let writeable = channel.writable().await;
        let bytes = match writeable {
            Ok(_) => {
                channel.try_write(&buf)?
            }
            Err(err) => {
                // todo: Flag DTR as false, in case we want to implement better error handling.
                return Err(err);
            }
        };

        Ok(bytes)
    }

    /// Flags the Data Terminal Ready pin, signifying whether the serial connection is ready for
    /// incoming data transmission.
    pub async fn flag_dtr(&mut self, ready: bool) -> Result<(), Box<dyn Error>> {
        Ok(self.channel.write_data_terminal_ready(ready)?)
    }
}