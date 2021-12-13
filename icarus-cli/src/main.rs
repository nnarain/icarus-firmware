//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 12 2021
//

use icarus_comms::{IcarusCommand, ppp::Transmitter};

use std::error::Error;

use clap::Parser;

/// Command line tool for interacting with icarus controller
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Serial port Icarus is connected to
    port: String,
    /// Serial baud rate
    #[clap(short = 'b', long = "baud", default_value_t = 115200)]
    baud: u32,
    /// Led state
    #[clap(short = 's', long = "state")]
    led_state: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Setup serial port
    let mut ser = serialport::new(args.port, args.baud).open()?;

    // Write data to the serial port
    let mut buf: [u8; 10] = [0; 10];
    
    let transmitter = Transmitter::default();
    let len = transmitter.encode(&mut buf, IcarusCommand::LedSet(args.led_state)).unwrap();

    println!("{:?}", &buf[..len]);

    ser.write(&mut buf[..len])?;

    Ok(())
}
