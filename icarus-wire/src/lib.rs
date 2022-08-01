#![no_std]
use serde::{Serialize, Deserialize};

use icarus_core::{
    EstimatedState, EstimatorInput,
};

// Re-export postcard functions for encoding and decoding
pub use postcard::{
    to_slice_cobs as encode,
    take_from_bytes_cobs as decode,
    to_vec_cobs as decode_vec,
    accumulator::{CobsAccumulator, FeedResult},
};

pub use postcard::{Result, Error};

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
pub enum IcarusState {
    Sensors(EstimatorInput),
    EstimatedState(EstimatedState),
    Battery(BatteryState),
}

/// Icarus command channels
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum IcarusCommand {
    Throttle(i8, i8, i8),
}
