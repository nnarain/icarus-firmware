//
// command.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 31 2022
//
use icarus_wire::{self, IcarusCommand};
use clap::Parser;

use tokio::{
    net::TcpStream
};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    cmd: Subcommand
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    Throttle {x_throttle: i8, y_throttle: i8, z_throttle: i8}
}

pub async fn run(args: Args, ip_addr: String) -> anyhow::Result<()> {
    let stream = TcpStream::connect(ip_addr).await?;

    let mut buf: [u8; 64] = [0; 64];

    match args.cmd {
        Subcommand::Throttle { x_throttle, y_throttle, z_throttle } => {
            let throttle_cmd = IcarusCommand::Throttle(x_throttle, y_throttle, z_throttle);
            if let Ok(used) = icarus_wire::encode(&throttle_cmd, &mut buf) {
                stream.writable().await?;
                if let Ok(_) = stream.try_write(used) {}
            }
        }
    }

    Ok(())
}
