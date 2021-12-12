//! no_std crate for sending and receiving Rust data structures over the serial port
//!
//!  Command packet structure
//! 
//! |  sync  | length | payload | crc     |
//! | 1 byte | 1 byte | N bytes | 2 bytes |
 
#![cfg_attr(not(test), no_std)]
// #![feature(generic_arg_infer)]
#![warn(missing_docs)]

mod error;
mod reader;
mod rx;

pub use error::Error;
pub use rx::ReceiveQueue;

pub use bincode;
pub use heapless::spsc as spsc;


