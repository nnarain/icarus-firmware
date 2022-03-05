//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 28 2021
//
#![no_std]

#[warn(missing_docs)]

mod icarus;
pub mod sensors;
pub mod types;

pub use cortex_m;
pub use cortex_m_rt as rt;

pub use rt::entry;

pub use stm32f3xx_hal as hal;

#[derive(Debug)]
pub enum IcarusError {
    HardwareInitError,
    SensorInitError,
    HseInitError,
}

pub mod specs {
    pub const HSI_FREQ: u32 = 8_000_000;
    pub const HSE_FREQ: u32 = 12_000_000;
    pub const SYSCLK_FREQ: u32 = 48_000_000;
}

pub mod prelude {
    pub use crate::hal::prelude::*;
    pub use crate::icarus::Icarus;
}
