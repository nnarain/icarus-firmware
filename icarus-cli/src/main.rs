//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 12 2021
//

// use icarus_comms::{IcarusCommand, ppp::Transmitter};
use icarus_cli::{
    cli::{Args, Action},
    // actions,
};
use tokio::{
    io::{self, AsyncRead},
    net::TcpStream
};
use icarus_wire::{IcarusState, CobsAccumulator, FeedResult};

use clap::Parser;

use std::time::Duration;

// fn main() -> Result<()> {
//     let args = Args::parse();

//     let port = args.port.as_str();
//     let baud = args.baud;
//     let timeout = args.timeout;

//     let ser = serialport::new(port, baud)
//                 .flow_control(serialport::FlowControl::None)
//                 .timeout(Duration::from_millis(timeout))
//                 .open()
//                 .with_context(|| format!("Failed to open serial port '{}'", port))?;

//     match args.action {
//         Action::Command => {},
//         Action::Monitor => {},
//         Action::Log(args) => actions::log::run(ser, args)?,
//     }

//     Ok(())
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let ip = &args.ip;
    let port = args.port;

    let ip_addr = format!("{}:{}", ip, port);

    let stream = TcpStream::connect(ip_addr).await?;

    let mut raw_buf: [u8; 1024] = [0; 1024];
    let mut cobs_buf: CobsAccumulator<256> = CobsAccumulator::new();

    loop {
        stream.readable().await?;

        match stream.try_read(&mut raw_buf) {
            Ok(0) => continue,
            Ok(n) => {
                let mut window = &raw_buf[..n];
                'cobs: while !window.is_empty() {
                    window = match cobs_buf.feed::<IcarusState>(window) {
                        FeedResult::Consumed => break 'cobs,
                        FeedResult::OverFull(new_window) => new_window,
                        FeedResult::DeserError(new_window) => new_window,
                        FeedResult::Success { data, remaining } => {
                            println!("{:?}", data);

                            remaining
                        }
                    }
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e.into()),
        }
    }


    Ok(())
}
