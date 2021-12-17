//
// cli.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 14 2021
//

use clap::Parser;
use crate::actions::log;

#[derive(Parser, Debug)]
pub enum Action {
    /// Read logs
    Log(log::Args),
    /// Send a command
    Command,
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
    #[clap(short = 'p', long = "port")]
    pub port: String,
    /// Serial baud rate
    #[clap(short = 'b', long = "baud", default_value_t = 115200)]
    pub baud: u32,
    /// Serial port timeout in milliseconds
    #[clap(short = 't', long = "timeout", default_value_t = 10)]
    pub timeout: u64,
}
