#![no_std]
use serde::{Serialize, Deserialize};

// Re-export postcard functions for encoding and decoding
pub use postcard::{
    to_slice_cobs as encode,
    take_from_bytes_cobs as decode,
};

pub use postcard::{Result, Error};

/// Raw data from the IMU
#[derive(Serialize, Deserialize, Debug)]
pub struct ImuRaw {
    pub accel: (f32, f32, f32),
    pub gyro: (f32, f32, f32),
    pub temp: f32,
}

/// IMU calibration offset
pub struct ImuCalibrationOffset {

}
/// Data reporting channels for Icarus
#[derive(Serialize, Deserialize, Debug)]
pub enum IcarusState<'a> {
    Log(&'a [u8]),
    ImuRaw(ImuRaw),
}

/// Icarus command channels
#[derive(Serialize, Deserialize, Debug)]
pub enum IcarusCommand {
    CycleLed,
}
