//
// log.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 14 2021
//

use icarus_wire::{self, IcarusState, IcarusCommand};

use anyhow::{Result, Context, anyhow, bail};
use clap::Parser;

use serialport::SerialPort;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;

use defmt_decoder::{DecodeError, Frame, Locations, Table};

type LocationInfo = (Option<String>, Option<u32>, Option<String>);


#[derive(Parser, Debug)]
pub struct Args {
    /// The ELF file of the application firmware the target is running
    elf: String,
    /// Verbosity level
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
    // #[clap(short = 's', long = "show-skipped-frames")]
    // show_skipped_frames: bool,
}

const READ_BUF_SIZE: usize = 1024;

/// Run icarus logger
/// See https://github.com/knurling-rs/defmt/blob/main/print/src/main.rs for defmt decoding
pub fn run(mut ser: Box<dyn SerialPort>, args: Args) -> Result<()> {
    let elf = args.elf.as_str();
    let verbose = args.verbose;
    // let show_skipped_frames = args.show_skipped_frames;

    defmt_decoder::log::init_logger(verbose, move |metadata| {
        if !verbose {
            defmt_decoder::log::is_defmt_frame(metadata)
        }
        else {
            true
        }
    });

    // Parse the ELF file for defmt decoder
    // let path = PathBuf::from(elf);
    // let bytes = fs::read(path).with_context(|| format!("Failed to read ELF file '{}'", elf))?;

    // let table = Table::parse(&bytes)?.ok_or_else(|| anyhow!(".defmt data not found"))?;
    // let locs = table.get_locations(&bytes)?;

    // let locs = if table.indices().all(|idx| locs.contains_key(&(idx as u64))) {
    //     Some(locs)
    // }
    // else {
    //     None
    // };

    // Setup signal handler
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Failed to interrupt signal"))?;

    // Current directory
    let current_dir = std::env::current_dir()?;

    // Serial receive buffer
    let mut buf: Vec<u8> = vec![0; READ_BUF_SIZE];
    // let mut stream_decoder = table.new_stream_decoder();

    loop {
        // Check if the user attempted to exit the program
        let exit = rx.try_recv();
        if exit.is_ok() {
            break;
        }

        match ser.read(buf.as_mut_slice()) {
            Ok(n) => {
                // stream_decoder.received(&buf[..n]);

                // match stream_decoder.decode() {
                //     Ok(frame) => forward_to_logger(&frame, location_info(&locs, &frame, &current_dir)),
                //     Err(DecodeError::UnexpectedEof) => break,
                //     Err(DecodeError::Malformed) => match table.encoding().can_recover() {
                //         false => return Err(DecodeError::Malformed.into()),
                //         true => {
                //             eprintln!("Frame Malformed, recoverable");
                //             continue;
                //         }
                //     }
                // }

                let mut remaining = Some(&mut buf[..n]);

                loop {
                    remaining = if let Some(bytes) = remaining {
                        match icarus_wire::decode::<IcarusState>(bytes) {
                            Ok((state, unused)) => {
                                println!("{:?}", state);
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

        let mut send_buf: [u8; 64] = [0; 64];
        let used_buf = icarus_wire::encode(&IcarusCommand::CycleLed, &mut send_buf).expect("Failed to encode");

        // ser.write_all(used_buf).expect("Failed to send");
        println!("Sending");
        // if let Ok(n) = ser.write(used_buf) {
        //     println!("sent: {}", n);
        // }
        // ser.w
    }

    Ok(())
}

fn forward_to_logger(frame: &Frame, location_info: LocationInfo) {
    let (file, line, mod_path) = location_info;
    defmt_decoder::log::log_defmt(frame, file.as_deref(), line, mod_path.as_deref());
}

fn location_info(locs: &Option<Locations>, frame: &Frame, current_dir: &Path) -> LocationInfo {
    let (mut file, mut line, mut mod_path) = (None, None, None);

    // NOTE(`[]` indexing) all indices in `table` have been verified to exist in the `locs` map
    let loc = locs.as_ref().map(|locs| &locs[&frame.index()]);

    if let Some(loc) = loc {
        // try to get the relative path, else the full one
        let path = loc.file.strip_prefix(&current_dir).unwrap_or(&loc.file);

        file = Some(path.display().to_string());
        line = Some(loc.line as u32);
        mod_path = Some(loc.module.clone());
    }

    (file, line, mod_path)
}
