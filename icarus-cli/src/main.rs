//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 12 2021
//

// use icarus_comms::{IcarusCommand, ppp::Transmitter};
use icarus_cli::{
    cli::{Args, Action},
    actions,
};

use clap::Parser;
use anyhow::{Result, Context};

use std::time::Duration;

fn main() -> Result<()> {
    let args = Args::parse();

    let port = args.port.as_str();
    let baud = args.baud;
    let timeout = args.timeout;

    let ser = serialport::new(port, baud)
                .flow_control(serialport::FlowControl::None)
                .timeout(Duration::from_millis(timeout))
                .open()
                .with_context(|| format!("Failed to open serial port '{}'", port))?;

    match args.action {
        Action::Command => {},
        Action::Monitor => {},
        Action::Log(args) => actions::log::run(ser, args)?,
    }

    Ok(())
}
