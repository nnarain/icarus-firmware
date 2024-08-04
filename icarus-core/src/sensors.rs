//
// sensors.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 13 2024
//

/// Orientation information
#[derive(Default, Debug, Clone, Copy)]
pub struct Attitude {
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
}

/// Complete sensor state of the drone
#[derive(Default, Debug, Clone, Copy)]
pub struct EstimatedState {
    pub attitude: Attitude,
}
