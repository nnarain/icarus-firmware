//
// log.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 14 2021
//

use icarus_wire::{self, IcarusState};

use anyhow::{Result, bail};
use clap::Parser;

use serialport::SerialPort;

use std::io;
use std::sync::mpsc::channel;

use chrono::prelude::*;

use csv::Writer;
use serde::Serialize;


#[derive(Parser, Debug)]
pub struct Args {
    /// CSV output
    #[clap(value_parser)]
    csv: String,
}

#[derive(Serialize, Debug)]
struct ImuRow {
    ts: String,
    ax: f32,
    ay: f32,
    az: f32,
    gx: f32,
    gy: f32,
    gz: f32,
}

const READ_BUF_SIZE: usize = 1024;

/// Run icarus logger
/// See https://github.com/knurling-rs/defmt/blob/main/print/src/main.rs for defmt decoding
pub fn run(mut ser: Box<dyn SerialPort>, args: Args) -> Result<()> {
    let csv_output_path = args.csv;
    let mut csv_writer = Writer::from_path(csv_output_path)?;

    // Setup signal handler
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Failed to interrupt signal"))?;

    // Serial receive buffer
    let mut buf: Vec<u8> = vec![0; READ_BUF_SIZE];

    loop {
        // Check if the user attempted to exit the program
        let exit = rx.try_recv();
        if exit.is_ok() {
            break;
        }

        match ser.read(buf.as_mut_slice()) {
            Ok(n) => {
                let mut remaining = Some(&mut buf[..n]);

                loop {
                    remaining = if let Some(bytes) = remaining {
                        match icarus_wire::decode::<IcarusState>(bytes) {
                            Ok((state, unused)) => {
                                match state {
                                    IcarusState::ImuRaw(imu) => {
                                        let ts: DateTime<Utc> = Utc::now();

                                        let row = ImuRow {
                                            ts: ts.to_string(),
                                            ax: imu.accel.0,
                                            ay: imu.accel.1,
                                            az: imu.accel.2,
                                            gx: imu.gyro.0,
                                            gy: imu.gyro.1,
                                            gz: imu.gyro.2,
                                        };
                                        csv_writer.serialize(row)?;
                                    },
                                    IcarusState::BarometerRaw(_) => {},
                                    IcarusState::Battery(state) => println!("{:?}", state),
                                    _ => {}
                                }
                                // unused.
                                Some(unused)
                            },
                            Err(_) => None,
                        }
                    }
                    else {
                        break;
                    }
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => bail!("{:?}", e),
        }
    }

    Ok(())
}
