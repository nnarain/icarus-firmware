
use std::time::Duration;

use icarus_cli::Args;
use clap::Parser;

use anyhow::Result;

fn main() -> Result<()> {
    let args = Args::parse();

    let mut ser = serialport::new(args.port, args.baud).timeout(Duration::from_millis(10)).open()?;

    let buf: [u8; 10] = [0x0f, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x00];

    ser.write(&buf[..])?;

    let mut rx_buf: [u8; 10] = [0; 10];
    ser.read(&mut rx_buf[..])?;

    println!("{:?}", rx_buf);

    Ok(())
}
