use std::error::Error;
use std::io::ErrorKind::WouldBlock;
use std::ops::Deref;
use std::thread::sleep;
use std::time::Duration;
use crate::core::serial::SerialChannel;

#[cfg(test)]
mod tests;

pub mod core;
pub mod interop;

#[tokio::main]
async fn main() {
    let mut channel = SerialChannel::from_config("./config.toml").unwrap();
    let mut buf = [0u8; 256];

    loop {
        let res = channel.read(buf.as_mut_slice()).await;

        match res {
            Ok(bytes) => {
                println!("{bytes} bytes read");
                let data = buf.to_vec();
                println!("{:?}", data);

                let _ = channel.respond();
            }
            Err(err) => {
                if err.kind() == WouldBlock {
                    async_std::task::sleep(Duration::from_secs(2)).await;
                    continue;
                } else {
                    eprintln!("{err}");
                    break;
                }
            }
        }

        buf = [0u8; 256];
    }

}
