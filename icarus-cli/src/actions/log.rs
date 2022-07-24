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
    let mut raw_buf: Vec<u8> = vec![0; READ_BUF_SIZE];
    let mut cobs_buf: CobsAccumulator<1024> = CobsAccumulator::new();

    loop {
        // Check if the user attempted to exit the program
        let exit = rx.try_recv();
        if exit.is_ok() {
            break;
        }

        match ser.read(raw_buf.as_mut_slice()) {
            Ok(n) => {
                println!("recv: {}", n);
                if n == 0 {
                    break;
                }

                let buf = &raw_buf[..n];
                let mut window = buf;

                'cobs: while !window.is_empty() {
                    // println!("window: {}", window.len());
                    window = match cobs_buf.feed::<IcarusState>(window) {
                        FeedResult::Consumed => {
                            println!("comsumed");
                            break 'cobs
                        },
                        FeedResult::OverFull(new_wind) => {
                            println!("overfull");
                            new_wind
                        },
                        FeedResult::DeserError(new_wind) => {
                            println!("error: {}", new_wind.len());
                            new_wind
                        },
                        FeedResult::Success { data, remaining } => {
                            println!("{:?} - {}", data, remaining.len());

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
