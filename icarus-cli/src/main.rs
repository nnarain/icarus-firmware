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
    io,
    signal,
    net::TcpStream
};
use icarus_wire::{IcarusState, CobsAccumulator, FeedResult};

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let ip = &args.ip;
    let port = args.port;

    let ip_addr = format!("{}:{}", ip, port);

    tokio::spawn(recv_task(ip_addr));

    // Wait to exit
    signal::ctrl_c().await?;

    Ok(())
}

async fn recv_task(ip_addr: String) -> anyhow::Result<()> {
    let stream = TcpStream::connect(ip_addr).await?;

    let mut raw_buf: [u8; 1024] = [0; 1024];
    let mut cobs_buf: CobsAccumulator<256> = CobsAccumulator::new();

    loop {
        stream.readable().await?;

        match stream.try_read(&mut raw_buf) {
            Ok(0) => break,
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
