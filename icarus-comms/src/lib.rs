//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 08 2021
//
#![no_std]
use serial_ppp::bincode::{self, Encode, Decode};

pub use serial_ppp as ppp;

#[derive(Encode, Decode, Debug)]
pub enum IcarusState {
    Heartbeat,
    Accel(f32, f32, f32),
    Gyro(f32, f32, f32),
}

#[derive(Encode, Decode, Debug, PartialEq)]
pub enum IcarusCommand {
    LedSet(bool),
}
