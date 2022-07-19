//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 10 2022
//

use icarus_app_std::{
    ImuCalibrationOffset,
    stat::{StatLed, StatColor},
};
use icarus_wire::{self, IcarusState, IcarusCommand, ImuRaw, BatteryState};

use esp_idf_hal::{
    prelude::*,
    peripherals::Peripherals,
    adc::{self, Adc, Analog, Atten11dB},
    ledc::*,
    i2c,
    delay::FreeRtos,
};
use embedded_hal_0_2::adc::{OneShot, Channel as _};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported


use mpu6050::Mpu6050;

use std::{
    sync::{
        Arc,
        mpsc::{channel, Receiver},
    },
    time::{Duration, SystemTime},
    thread,
    io::{Write, Read}
};

// Voltage Dividor
//   R1 = 6.8k
//   R2 = 10k
//
// Ratio = R1 / (R1 + R2)
const BATTERY_SENSE_DIVIDOR_RATIO: f32 = 0.5952381;

#[allow(unreachable_code)]
fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // -----------------------------------------------------------------------------------------------------------------
    // Setup Logging
    // -----------------------------------------------------------------------------------------------------------------

    // let mut logger = defmt_bbq::init().unwrap();

    // -----------------------------------------------------------------------------------------------------------------
    // Hardware Init
    // -----------------------------------------------------------------------------------------------------------------
    // defmt::info!("Setting up hardware");

    let p = Peripherals::take().unwrap();

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

    // Battery sense
    let adc_config = adc::config::Config::default();
    let mut adc_channel = adc::PoweredAdc::new(p.adc1, adc_config)?;
    
    let mut battery_sense = p.pins.gpio3.into_analog_atten_11db()?;

    // -----------------------------------------------------------------------------------------------------------------
    // Tasks
    // -----------------------------------------------------------------------------------------------------------------

    // Setup task queues
    let (cmd_sender, cmd_reciever) = channel::<IcarusCommand>();

    let (state_sender, state_reciever) = channel::<IcarusState>();
    let imu_state_sender = state_sender.clone();

    let (battery_sender, battery_reciever) = channel::<IcarusState>();

    // Spawn command task
    thread::spawn(move || {
        let mut read_buf: [u8; 64] = [0; 64];
        loop {
            // TODO: Clean up
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
        let offsets = calibrate_imu(500, 20, || {
            let a = imu.get_acc().unwrap();
            let g = imu.get_gyro().unwrap();

            ((a.x, a.y, a.z), (g.x, g.y, g.z))
        });

        let mut battery_last_read_time = SystemTime::now();

        loop {
            // Read IMU data
            let accel = imu.get_acc();
            let gyro = imu.get_gyro();
            let temp = imu.get_temp();

            if let (Ok(accel), Ok(gyro), Ok(temp)) = (accel, gyro, temp) {
                // TODO: Actual state estimation
                let imu_raw = ImuRaw {
                    accel: (
                        accel.x - offsets.ax_offset,
                        accel.y - offsets.ay_offset,
                        accel.z - offsets.az_offset,
                    ),
                    gyro: (
                        gyro.x - offsets.gx_offset,
                        gyro.y - offsets.gy_offset,
                        gyro.z - offsets.gz_offset,
                    ),
                    temp,
                };
                let imu_raw = IcarusState::ImuRaw(imu_raw);

                imu_state_sender.send(imu_raw).expect("Failed to send IMU data");
            }

            // Read battery voltage every 1000ms and report to host
            let now = SystemTime::now();
            let should_read_battery = now
                                        .duration_since(battery_last_read_time)
                                        .map(|e| e >= Duration::from_millis(1000));
            if let Ok(true) = should_read_battery {
                let adc_raw = nb::block!(adc_channel.read(&mut battery_sense));
                battery_last_read_time = now;

                if let Ok(adc_raw) = adc_raw {
                    let voltage = ((adc_raw as f32) / BATTERY_SENSE_DIVIDOR_RATIO) as u16;
                    let battery_state = BatteryState {voltage, adc_raw, charge_complete: false};
                    battery_sender.send(IcarusState::Battery(battery_state)).expect("Failed to send battery state");
                }
            }

            thread::sleep(Duration::from_millis(20));
        }
    });

    // Spawn LED task
    thread::spawn(move || {
        let colors: [StatColor; 3] = [StatColor::Red, StatColor::Green, StatColor::Blue];

        for c in colors.iter().cycle() {
            stat_led.update(*c).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });


    defmt::info!("Starting IDLE task");

    let mut send_buf: [u8; 64] = [0; 64];

    // Idle Task
    loop {
        // Write all recieved state to serial port
        let all_recv = state_reciever.try_iter().chain(battery_reciever.try_iter());

        for state in all_recv {
            let used_buf = icarus_wire::encode(&state, &mut send_buf)?;
            std::io::stdout().write_all(&used_buf)?;
            std::io::stdout().flush()?;
            // TODO(nnarain): Why is this needed?
            println!("\n\r");
        }

        // Recieve commands and dispatch to tasks
        for cmd in cmd_reciever.try_iter() {
            match cmd {
                IcarusCommand::CycleLed => {},
            }
        }

        // Write log chunks to serial port
        // match logger.read() {
        //     Ok(grant) => {
        //         let buf = grant.buf();
        //         let log_chunk = IcarusState::Log(buf);

        //         let used_buf = icarus_wire::encode(&log_chunk, &mut send_buf)?;
        //         std::io::stdout().write_all(&used_buf)?;
        //         std::io::stdout().flush()?;
        //         // TODO(nnarain): Why is this needed?
        //         println!("\n\r");

        //         let len = grant.len();
        //         grant.release(len);
        //     },
        //     Err(e) => println!("{:?}", e),
        // }

        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

/// Sample accelerometer and gyro data and calculate the device specific offset
fn calibrate_imu<F>(samples: usize, delay_ms: u64, mut f: F) -> ImuCalibrationOffset
    where F: FnMut() -> ((f32, f32, f32), (f32, f32, f32)) {

    let (a, g) = f();

    // Min / Max values for each axis on the accelerometer and the gyro
    let mut ax_min: f32 = a.0;
    let mut ax_max: f32 = a.0;
    let mut ay_min: f32 = a.1;
    let mut ay_max: f32 = a.1;
    let mut az_min: f32 = a.2;
    let mut az_max: f32 = a.2;
    let mut gx_min: f32 = g.0;
    let mut gx_max: f32 = g.0;
    let mut gy_min: f32 = g.1;
    let mut gy_max: f32 = g.1;
    let mut gz_min: f32 = g.2;
    let mut gz_max: f32 = g.2;

    for _ in 0..samples {
        let (a, g) = f();

        ax_min = ax_min.min(a.0);
        ax_max = ax_max.max(a.0);
        ay_min = ay_min.min(a.1);
        ay_max = ay_max.max(a.1);
        az_min = az_min.min(a.2);
        az_max = az_max.max(a.2);

        gx_min = gx_min.min(g.0);
        gx_max = gx_max.max(g.0);
        gy_min = gy_min.min(g.1);
        gy_max = gy_max.max(g.1);
        gz_min = gz_min.min(g.2);
        gz_max = gz_max.max(g.2);

        thread::sleep(Duration::from_millis(delay_ms))
    }

    ImuCalibrationOffset {
        ax_offset: (ax_max - ax_min) / 2.0 + ax_min,
        ay_offset: (ay_max - ay_min) / 2.0 + ay_min,
        az_offset: (az_max - az_min) / 2.0 + az_min,
        gx_offset: (gx_max - gx_min) / 2.0 + gx_min,
        gy_offset: (gy_max - gy_min) / 2.0 + gy_min,
        gz_offset: (gz_max - gz_min) / 2.0 + gz_min,
    }
}

// fn send_to_host(recv: &mut Receiver<IcarusState>) -> anyhow::Result<()> {
//     let state = recv.try_iter()

//     Ok(())
// }
