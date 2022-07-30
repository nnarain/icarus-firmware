//
// console.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 28 2022
//

use getargs::Options;

/// Commands recieved from the serial console
pub enum ConsoleCommand {
    Wireless(WirelessCommands),
}

/// Port to bind
struct StringBuf([u8; 32], u8);

/// Commands related to wireless communication
pub enum WirelessCommands {
    /// Set the port to bind to
    Set,
    /// Print wireless info to console
    Get,
}

/// Consume a byte buffer (line delimited) and parse command options
pub fn parse(bytes: &[u8]) -> Option<ConsoleCommand> {
    let mut opts = Options::new(bytes.split(|&b| b == b' '));

    let subcommand = opts.next_positional();

    match subcommand {
        Some(b"wireless") => {
            let wireless_subcommand = opts.next_positional();
            match wireless_subcommand {
                Some(b"get") => Some(ConsoleCommand::Wireless(WirelessCommands::Get)),
                Some(_) | None => None,
            }
        }
        Some(_) | None => None,
    }

}
