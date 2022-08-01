//
// cli.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 14 2021
//

use clap::Parser;
use crate::actions::{log, command};

#[derive(Parser, Debug)]
pub enum Action {
    /// Read logs
    Log(log::Args),
    /// Send a command
    Command(command::Args),
    /// Monitor system state
    Monitor,
}

/// Command line tool for interacting with icarus controller
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
    /// Serial port Icarus is connected to
    #[clap(short = 'i', long = "ip")]
    pub ip: String,
    /// Serial baud rate
    #[clap(short = 'p', long = "port", default_value_t = 5000)]
    pub port: u16,
}
