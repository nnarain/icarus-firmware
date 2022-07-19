#![no_std]
use serde::{Serialize, Deserialize};

// Re-export postcard functions for encoding and decoding
pub use postcard::{
    to_slice_cobs as encode,
    take_from_bytes_cobs as decode,
    to_vec_cobs as decode_vec,
};

pub use postcard::{Result, Error};

/// Raw data from the IMU
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ImuRaw {
    pub accel: (f32, f32, f32),
    pub gyro: (f32, f32, f32),
    pub temp: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct BarometerRaw {
    pub altitude: f32,
    pub temp: f32,
}

/// IMU calibration offset
pub struct ImuCalibrationOffset {}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct BatteryState {
    /// Battery voltage
    pub voltage: u16,
    /// Raw value from ADC
    pub adc_raw: u16,
    /// Charge state from the LiPo charger
    pub charge_complete: bool,
}

/// Data reporting channels for Icarus
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum IcarusState<'a> {
    Log(&'a [u8]),
    ImuRaw(ImuRaw),
    BarometerRaw(BarometerRaw),
    Battery(BatteryState),
}

/// Icarus command channels
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum IcarusCommand {
    CycleLed,
}
