//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 28 2021
//
#![no_std]

pub use cortex_m;
pub use cortex_m_rt as rt;

pub use rt::entry;

pub use stm32f3xx_hal as hal;

pub mod prelude {
    pub use crate::hal::prelude::*;
}
