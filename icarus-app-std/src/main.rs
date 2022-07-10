//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 10 2022
//

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::{
    prelude::*,
    peripherals::Peripherals,
    ledc::*,
    i2c,
    delay::FreeRtos,
};
use icarus_app_std::stat::{StatLed, StatColor};

use embedded_hal::i2c::blocking::I2c;

use mpu6050::{Mpu6050, Mpu6050Error};

use std::{borrow::Borrow, sync::Arc, time::Duration, thread};

#[allow(unreachable_code)]
fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let mut p = Peripherals::take().unwrap();

    // -----------------------------------------------------------------------------------------------------------------
    // Hardware Init
    // -----------------------------------------------------------------------------------------------------------------

    // Rotor control
    let drv1_en = p.pins.gpio10.into_output()?;
    let drv2_en = p.pins.gpio6.into_output()?;

    let config = config::TimerConfig::default().frequency(50.Hz().into());
    let timer = Arc::new(Timer::new(p.ledc.timer0, &config)?);

    let rtrctl1 = Channel::new(p.ledc.channel0, timer.clone(), p.pins.gpio8)?;
    let rtrctl2 = Channel::new(p.ledc.channel1, timer.clone(), p.pins.gpio7)?;
    let rtrctl3 = Channel::new(p.ledc.channel2, timer.clone(), p.pins.gpio5)?;
    let rtrctl4 = Channel::new(p.ledc.channel3, timer.clone(), p.pins.gpio4)?;

    // GPIO
    let user_button = p.pins.gpio9.into_input()?;

    // Stat LED
    let mut stat_led = StatLed::new(p.pins.gpio21, p.rmt.channel0)?;

    // Sensors
    let sda = p.pins.gpio1;
    let scl = p.pins.gpio2;

    let i2c_config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());
    let i2c = i2c::Master::<i2c::I2C0, _, _>::new(p.i2c0, i2c::MasterPins {sda, scl}, i2c_config)?;

    let mut delay = FreeRtos{};

    let mut imu = Mpu6050::new_with_addr(i2c, 0x68);
    imu.init(&mut delay).unwrap(); // TODO: MPU6050 error type does not implement `Error` (can't use `?` operator)

    // TODO(nnarain): Barometer

    // -----------------------------------------------------------------------------------------------------------------
    // Tasks
    // -----------------------------------------------------------------------------------------------------------------

    // TODO...

    let colors: [StatColor; 3] = [StatColor::Red, StatColor::Green, StatColor::Blue];
    let mut c = 0;

    // Idle Task
    loop {
        let accel_data = imu.get_acc().unwrap();
        let gyro_data = imu.get_gyro().unwrap();

        println!("A: {:?}, G: {:?}", accel_data, gyro_data);

        stat_led.update(colors[c])?;
        c = (c + 1) % colors.len();

        // TODO(nnarain): Write state updates to serial port
        thread::sleep(Duration::from_millis(1000u64));
    }

    Ok(())
}
