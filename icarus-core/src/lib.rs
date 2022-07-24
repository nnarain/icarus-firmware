//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 22 2022
//
#![no_std]
pub mod data;
pub mod filter;

use crate::{
    data::{AccelerometerData, GyroscopeData, Attitude},
    filter::TriAxialFilter,
};

use serde::{Serialize, Deserialize};
use ahrs::{Ahrs, Madgwick};
use nalgebra::Vector3;

pub enum EstimatorError {
    AhrsError,
}

/// Estimated state
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct EstimatedState {
    /// Orientation
    pub attitude: Attitude,
    /// Estimated velocity (TODO: vector type?)
    pub z_vel: f32,
}

/// Input data for the state estimator
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct EstimatorInput {
    pub accel: AccelerometerData,
    pub gyro: GyroscopeData,
    pub altitude: f32,
}

/// Consume Accelerometer, Gyro, Magetometer, Barometer data and determine system state
pub struct StateEstimator {
    /// AHRS filter
    ahrs: Madgwick<f32>,
    /// Accelerometer filter. 3 samples.
    accel_filter: TriAxialFilter<3>,
    /// Gyro filter. 3 samples
    gyro_filter: TriAxialFilter<3>,
}

impl Default for StateEstimator {
    fn default() -> Self {
        StateEstimator {
            ahrs: Madgwick::new(0.02, 0.1),
            accel_filter: TriAxialFilter::default(),
            gyro_filter: TriAxialFilter::default(),
        }
    }
}

impl StateEstimator {
    pub fn update(&mut self, input: EstimatorInput, delta: f32) -> Result<EstimatedState, EstimatorError> {
        let EstimatorInput{accel, gyro, altitude: _} = input;

        // Filter raw IMU data
        self.accel_filter.update(accel);
        self.gyro_filter.update(gyro);

        let accel = self.accel_filter.value();
        let gyro = self.gyro_filter.value();

        let accel = Vector3::new(accel.0, accel.1, accel.2);
        let gyro = Vector3::new(gyro.0, gyro.1, gyro.2);

        // self.ahrs.update(&gyro, &acccel, magnetometer)
        let sample_period = self.ahrs.sample_period_mut();
        *sample_period = delta;

        let quat = self.ahrs.update_imu(&gyro, &accel).map_err(|_| EstimatorError::AhrsError)?;
        let (roll, pitch, yaw) = quat.euler_angles();

        Ok(EstimatedState{
            attitude: Attitude { pitch, roll, yaw },
            z_vel: 0.0
        })
    }
}
