//
// data.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 22 2022
//
use serde::{Serialize, Deserialize};

// pub trait TriAxialData {
//     fn value(self) -> (f32, f32, f32);
// }

/// Raw accelerometer data
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct AccelerometerData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<AccelerometerData> for (f32, f32, f32) {
    fn from(a: AccelerometerData) -> Self {
        (a.x, a.y, a.x)
    }
}

/// Raw gyroscope data
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct GyroscopeData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<GyroscopeData> for (f32, f32, f32) {
    fn from(g: GyroscopeData) -> Self {
        (g.x, g.y, g.x)
    }
}

/// Raw IMU data
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ImuData {
    pub accel: AccelerometerData,
    pub gyro: GyroscopeData,
}

/// Magnetometer data
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct MagnetometerData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<MagnetometerData> for (f32, f32, f32) {
    fn from(m: MagnetometerData) -> Self {
        (m.x, m.y, m.z)
    }
}

/// Orientation - Pitch, Roll, Yaw
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct Attitude {
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
}
