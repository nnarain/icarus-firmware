//
// imu.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 30 2021
//
#![no_std]
#![no_main]

use panic_halt as _;

use icarus::{
    entry,
    prelude::*,
    sensors::imu,
    cortex_m::asm,
};

use core::fmt::Write;

#[entry]
fn main() -> ! {
    let icarus = Icarus::take().unwrap();
    let i2c = icarus.i2c;
    let mut delay = icarus.delay;
    let mut serial = icarus.usart1;

    let mut imu = imu::init(i2c.acquire_i2c());
    imu.init(&mut delay).unwrap();

    loop {
        // Accelerometer
        let acc = imu.get_acc().unwrap();
        write!(serial, "Accel: {:?}\r\n", acc).unwrap();

        // Gyroscope
        let gyro = imu.get_gyro().unwrap();
        write!(serial, "Gyro: {:?}\r\n", gyro).unwrap();

        // Roll pitch estimate
        let ori = imu.get_acc_angles().unwrap();
        write!(serial, "Ori: {:?}]\r\n", ori).unwrap();

        let temperature = imu.get_temp().unwrap();
        write!(serial, "Temperature: {:.2}\r\n", temperature).unwrap();

        asm::delay(8_000_000);
    }
}
