//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 12 2021
//

use icarus_comms::{IcarusCommand, ppp::bincode};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf: [u8; 10] = [0; 10];
    let len = bincode::encode_into_slice(
        IcarusCommand::LedSet(true),
        &mut buf,
        bincode::config::Configuration::standard()
    ).unwrap();

    println!("{}, {:?}", len, buf);

    Ok(())
}
