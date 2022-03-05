//
// barometer.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Mar 03 2022
//
#![no_std]
#![no_main]

use panic_halt as _;

use icarus::{
    entry,
    prelude::*,
    sensors::barometer,
    cortex_m::asm,
};

use core::fmt::Write;

#[entry]
fn main() -> ! {
    let hw = Icarus::take().unwrap();
    let i2c = hw.i2c;
    let mut serial = hw.usart1;

    let mut barometer = barometer::init(i2c.acquire_i2c()).unwrap();

    loop {
        if let Ok(data) = barometer.sensor_values() {
            write!(serial, "{:?}\r\n", data).unwrap();
        }
        asm::delay(8_000_000);
    }
}
