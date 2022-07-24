//
// log.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 14 2021
//

use icarus_wire::{self, IcarusState, CobsAccumulator, FeedResult};

use anyhow::{Result, bail};
use clap::Parser;

use serialport::SerialPort;

use std::{
    io,
    path::PathBuf,
    sync::mpsc::channel,
    time::{Duration, Instant}, str::FromStr,
};

use chrono::prelude::*;

use serde::Serialize;


#[derive(Parser, Debug)]
pub struct Args {
    /// CSV output directory
    #[clap(value_parser, default_value = "output")]
    output_dir: String,
}

#[derive(Serialize, Debug)]
struct SensorRow {
    ts: f32,
    ax: f32,
    ay: f32,
    az: f32,
    gx: f32,
    gy: f32,
    gz: f32,
}

// impl TryFrom<IcarusState> for SensorRow {
//     fn try_from(value: IcarusState) -> Result<Self, Self::Error> {
//         if let IcarusState::Sensors(sensors) =  {
            
//         }
//         else {
//             Err(())
//         }
//     }
// }

#[derive(Serialize, Debug)]
struct AttitudeRow {
    ts: f32,
    pitch: f32,
    roll: f32,
    yaw: f32,
}

const READ_BUF_SIZE: usize = 1024;

/// Run icarus logger
/// See https://github.com/knurling-rs/defmt/blob/main/print/src/main.rs for defmt decoding
pub fn run(mut ser: Box<dyn SerialPort>, args: Args) -> Result<()> {
    let out_dir = PathBuf::from_str(&args.output_dir)?;

    let sensors_path = out_dir.join("sensors.csv");
    let attitude_path = out_dir.join("attitude.csv");

    let mut sensors_writer = csv::Writer::from_path(sensors_path)?;
    let mut attitude_writer = csv::Writer::from_path(attitude_path)?;

    // Setup signal handler
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Failed to interrupt signal"))?;

    // Serial receive buffer
    let mut raw_buf: Vec<u8> = vec![0; READ_BUF_SIZE];
    let mut filter_buf: Vec<u8> = vec![0; READ_BUF_SIZE];

    let mut cobs_buf: CobsAccumulator<256> = CobsAccumulator::new();

    let start = Instant::now();

    loop {
        // Check if the user attempted to exit the program
        let exit = rx.try_recv();
        if exit.is_ok() {
            break;
        }

        match ser.read(raw_buf.as_mut_slice()) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                let now = Instant::now();

                let filtered_len = filter_data(filter_buf.as_mut_slice(), &raw_buf[..n]);

                let buf = &filter_buf[..filtered_len];
                let mut window = buf;

                'cobs: while !window.is_empty() {
                    window = match cobs_buf.feed::<IcarusState>(window) {
                        FeedResult::Consumed => {
                            break 'cobs
                        },
                        FeedResult::OverFull(new_wind) => {
                            new_wind
                        },
                        FeedResult::DeserError(new_wind) => {
                            new_wind
                        },
                        FeedResult::Success { data, remaining } => {
                            // println!("{:?}", data);

                            match data {
                                IcarusState::Sensors(sensors) => {
                                    let imu_row = SensorRow {
                                        ts: now.duration_since(start).as_secs_f32(),
                                        ax: sensors.accel.x,
                                        ay: sensors.accel.y,
                                        az: sensors.accel.z,
                                        gx: sensors.gyro.x,
                                        gy: sensors.gyro.y,
                                        gz: sensors.gyro.z,
                                    };
                                    sensors_writer.serialize(imu_row)?;
                                },
                                IcarusState::EstimatedState(state) => {
                                    let attitude_row = AttitudeRow {
                                        ts: now.duration_since(start).as_secs_f32(),
                                        pitch: state.attitude.pitch,
                                        roll: state.attitude.roll,
                                        yaw: state.attitude.yaw,
                                    };
                                    attitude_writer.serialize(attitude_row)?;
                                },
                                _ => {}
                            }

                            remaining
                        }
                    }
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => bail!("{:?}", e),
        }
    }

    Ok(())
}

// fn write_to_output<W: io::Write, D: Serialize>(writer: W, data: D) {
//     writer
// }

// Well this doesn't feel very Rust-y....
fn filter_data(out: &mut [u8], recv: &[u8]) -> usize {
    let mut delimiter_found = false;
    let mut idx = 0;

    for &b in recv {
        if !delimiter_found {
            out[idx] = b;
            idx += 1;
        }
        else if b == b'\n' {
            delimiter_found = false;
        }

        if b == 0 {
            delimiter_found = true;
        }
    }

    idx
}
