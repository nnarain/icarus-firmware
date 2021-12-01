//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 28 2021
//
#![no_std]
mod icarus;
pub mod sensors;

pub use cortex_m;
pub use cortex_m_rt as rt;

pub use rt::entry;

pub use stm32f3xx_hal as hal;

#[derive(Debug)]
pub enum IcarusError {
    HardwareInitError
}

pub mod prelude {
    pub use crate::hal::prelude::*;
    pub use crate::icarus::Icarus;
}
