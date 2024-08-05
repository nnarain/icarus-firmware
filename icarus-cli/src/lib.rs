//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 16 2024
//

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Serial port
    pub port: String,
    /// Baud rate
    #[arg(short, long, default_value_t = 115200)]
    pub baud: u32,
}
