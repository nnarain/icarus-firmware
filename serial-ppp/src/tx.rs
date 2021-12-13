//
// tx.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 12 2021
//

use crate::{Error, SYNC};
use bincode::Encode;

/// Encode a value in a slice
#[derive(Default)]
pub struct Transmitter {
}

impl Transmitter {
    /// Encode the message in the specified buffer
    pub fn encode<Msg: Encode>(&self, buf: &mut [u8], msg: Msg) -> Result<usize, Error> {
        buf[0] = SYNC;

        let config = bincode::config::Configuration::standard();
        let len = bincode::encode_into_slice(msg, &mut buf[2..], config).map_err(|_| Error::EncodeError)?;

        buf[1] = len as u8;

        // TODO: CRC
        buf[2 + len] = 0xFF;
        buf[2 + len + 1] = 0xFF;

        Ok(len + 1 + 1 + 2)
    }
}
