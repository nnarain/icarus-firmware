#![no_std]
//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 11 2024
//

// Re-exports
pub use icarus_core::{rc, sensors};


/// Task queues
pub mod queues {
    use super::*;

    use rc::RcInput;
    use sensors::SensorState;

    use embassy_sync::{
        blocking_mutex::raw::ThreadModeRawMutex,
        channel::{Channel, Sender, Receiver}
    };

    pub type RcInputChannel = Channel<ThreadModeRawMutex, RcInput, 1>;
    pub type RcInputChannelSender = Sender<'static, ThreadModeRawMutex, RcInput, 1>;
    pub type RcInputChannelReceiver = Receiver<'static, ThreadModeRawMutex, RcInput, 1>;

    pub type SensorStateChannel = Channel<ThreadModeRawMutex, SensorState, 1>;
    pub type SensorStateChannelSender = Sender<'static, ThreadModeRawMutex, SensorState, 1>;
    pub type SensorStateChannelReceiver = Receiver<'static, ThreadModeRawMutex, SensorState, 1>;
}
