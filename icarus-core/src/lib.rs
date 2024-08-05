//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 13 2024
//
#![cfg_attr(not(test), no_std)]
#![feature(const_fn_floating_point_arithmetic)]

pub mod rc;
pub mod sensors;
pub mod telemetry;


// 200Hz -> 20ms
// 16-bit resolution -> 65535 steps
// 20ms / 65535 -> 3.05e-7 ms per step
//
// Max Throttle -> 2ms pulse width
// 2ms / 3.05e-7 = 6553
//
// Min Throttle -> 1ms pulse width
// 1ms / 3.05e-7 = 3276

pub const ROTOR_PWM_FREQ: u32 = 50;
pub const MIN_ROTOR_THROTTLE: u16 = 3276;
pub const MAX_ROTOR_THROTTLE: u16 = 6553;
