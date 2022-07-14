//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 10 2022
//

use icarus_app_std::stat::{StatLed, StatColor};
use icarus_wire::{self, IcarusState, IcarusCommand, ImuRaw};

use esp_idf_hal::{
    prelude::*,
    peripherals::Peripherals,
    ledc::*,
    i2c,
    delay::FreeRtos,
};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported


use mpu6050::Mpu6050;

use std::{
    sync::{
        Arc,
        mpsc::channel,
    },
    time::Duration,
    thread,
    io::{Write, Read}
};

#[allow(unreachable_code)]
fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // -----------------------------------------------------------------------------------------------------------------
    // Setup Logging
    // -----------------------------------------------------------------------------------------------------------------

    // TODO(nnarain): This library depends on cortex-m crate
    // let mut logger = defmt_bbq::init().unwrap();

    // -----------------------------------------------------------------------------------------------------------------
    // Hardware Init
    // -----------------------------------------------------------------------------------------------------------------
    let mut p = Peripherals::take().unwrap();

    // Rotor control
    let _drv1_en = p.pins.gpio10.into_output()?;
    let _drv2_en = p.pins.gpio6.into_output()?;

    let config = config::TimerConfig::default().frequency(50.Hz().into());
    let timer = Arc::new(Timer::new(p.ledc.timer0, &config)?);

    let _rtrctl1 = Channel::new(p.ledc.channel0, timer.clone(), p.pins.gpio8)?;
    let _rtrctl2 = Channel::new(p.ledc.channel1, timer.clone(), p.pins.gpio7)?;
    let _rtrctl3 = Channel::new(p.ledc.channel2, timer.clone(), p.pins.gpio5)?;
    let _rtrctl4 = Channel::new(p.ledc.channel3, timer.clone(), p.pins.gpio4)?;

    // GPIO
    let _user_button = p.pins.gpio9.into_input()?;

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

    // Setup task queues
    let (cmd_sender, cmd_reciever) = channel::<IcarusCommand>();

    let (state_sender, state_reciever) = channel::<IcarusState>();
    let imu_state_sender = state_sender.clone();

    let (led_sender, led_reciever) = channel::<IcarusCommand>();

    // Spawn command task
    thread::spawn(move || {
        let mut read_buf: [u8; 64] = [0; 64];
        loop {
            match std::io::stdin().read(&mut read_buf) {
                Ok(n) => {
                    let mut remaining = Some(&mut read_buf[..n]);

                    loop {
                        remaining = if let Some(bytes) = remaining {
                            match icarus_wire::decode::<IcarusCommand>(bytes) {
                                Ok((cmd, unused)) => {
                                    cmd_sender.send(cmd).expect("Failed to send recieved command on channel");
                                    Some(unused)
                                },
                                Err(_) => None,
                            }
                        }
                        else {
                            break;
                        }
                    }
                },
                Err(_) => {},
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Spawn sensors task
    thread::spawn(move || {
        loop {
            let accel = imu.get_acc();
            let gyro = imu.get_gyro();
            let temp = imu.get_temp();

            if let (Ok(accel), Ok(gyro), Ok(temp)) = (accel, gyro, temp) {
                // TODO: Actual state estimation
                let imu_raw = ImuRaw {
                    accel: (accel.x, accel.y, accel.z),
                    gyro: (gyro.x, gyro.y, gyro.z),
                    temp,
                };
                let imu_raw = IcarusState::ImuRaw(imu_raw);

                imu_state_sender.send(imu_raw).expect("Failed to send IMU data");
            }

            thread::sleep(Duration::from_millis(1000));
        }
    });

    // Spawn LED task
    thread::spawn(move || {
        let colors: [StatColor; 3] = [StatColor::Red, StatColor::Green, StatColor::Blue];
        let mut c = 0;

        loop {
            // stat_led.update(colors[c]).unwrap();
            // c = (c + 1) % colors.len();

            // thread::sleep(Duration::from_millis(1000));
            match led_reciever.recv() {
                Ok(cmd) => {
                    stat_led.update(colors[c]).unwrap();
                    c = (c + 1) % colors.len();
                },
                Err(_) => {}
            }
        }
    });

    let mut send_buf: [u8; 64] = [0; 64];

    // Idle Task
    loop {
        // Write all recieved state to serial port
        loop {
            match state_reciever.try_recv() {
                Ok(state) => {
                    let used_buf = icarus_wire::encode(&state, &mut send_buf)?;
                    std::io::stdout().write_all(&used_buf)?;

                    std::io::stdout().flush()?;
                    // TODO(nnarain): Why is this needed?
                    println!("\n\r");
                },
                Err(_) => {
                    break;
                },
            }
        }

        // Recieve commands and dispatch to tasks
        loop {
            match cmd_reciever.try_recv() {
                Ok(cmd) => {
                    match cmd {
                        IcarusCommand::CycleLed => led_sender.send(cmd.clone()).unwrap(),
                    }
                },
                Err(_) => break,
            }
        }

        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
