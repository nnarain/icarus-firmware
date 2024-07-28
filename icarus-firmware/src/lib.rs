#![no_std]
//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 11 2024
//

// Re-exports
pub use icarus_core::{rc, sensors, telemetry};


/// Task queues
pub mod queues {
    use super::*;

    use rc::RcInput;
    use sensors::EstimatedState;
    use telemetry::Telemetry;

    use embassy_sync::{
        blocking_mutex::raw::ThreadModeRawMutex,
        channel::{Channel, Sender, Receiver}
    };

    // TODO(nnarain): Macro?

    pub type RcInputChannel = Channel<ThreadModeRawMutex, RcInput, 1>;
    pub type RcInputChannelSender = Sender<'static, ThreadModeRawMutex, RcInput, 1>;
    pub type RcInputChannelReceiver = Receiver<'static, ThreadModeRawMutex, RcInput, 1>;

    pub type EstimatedStateChannel = Channel<ThreadModeRawMutex, EstimatedState, 1>;
    pub type EstimatedStateChannelSender = Sender<'static, ThreadModeRawMutex, EstimatedState, 1>;
    pub type EstimatedStateChannelReceiver = Receiver<'static, ThreadModeRawMutex, EstimatedState, 1>;

    pub type TelemetryChannel = Channel<ThreadModeRawMutex, Telemetry, 1>;
    pub type TelemetryChannelSender = Sender<'static, ThreadModeRawMutex, Telemetry, 1>;
    pub type TelemetryChannelReceiver = Receiver<'static, ThreadModeRawMutex, Telemetry, 1>;
}
