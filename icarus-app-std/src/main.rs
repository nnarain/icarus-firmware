//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 10 2022
//

use icarus_app_std::{
    stat::{StatColor, StatLed},
    wifi::AppWifi,
    console::{self, ConsoleCommand, WirelessCommands},
    ImuCalibrationOffset,
};
use icarus_core::{
    data::{AccelerometerData, GyroscopeData},
    EstimatorInput, StateEstimator,
};
use icarus_wire::{self, IcarusCommand, IcarusState};

use esp_idf_hal::{delay::FreeRtos, i2c, ledc::*, peripherals::Peripherals, prelude::*};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use mpu6050::Mpu6050;

use std::{
    io::{Read, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::channel,
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

const WIFI_SSID: &str = env!("ICARUS_WIFI_SSID");
const WIFI_PASS: &str = env!("ICARUS_WIFI_PASS");

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
    let i2c =
        i2c::Master::<i2c::I2C0, _, _>::new(p.i2c0, i2c::MasterPins { sda, scl }, i2c_config)?;

    let mut delay = FreeRtos {};

    let mut imu = Mpu6050::new_with_addr(i2c, 0x68);
    imu.init(&mut delay).unwrap(); // TODO: MPU6050 error type does not implement `Error` (can't use `?` operator)

    // -----------------------------------------------------------------------------------------------------------------
    // Wireless Setup
    // -----------------------------------------------------------------------------------------------------------------

    // Setup WiFi (in the future this will be Bluetooth LE)
    let mut wifi = AppWifi::new()?;
    wifi.connect(WIFI_SSID, WIFI_PASS)?;

    // TODO(nnarain): Barometer

    // -----------------------------------------------------------------------------------------------------------------
    // Tasks
    // -----------------------------------------------------------------------------------------------------------------

    // Setup task queues and shared state
    let (cmd_sender, cmd_reciever) = channel::<IcarusCommand>();
    let (state_sender, state_reciever) = channel::<IcarusState>();

    let (console_cmd_sender, console_cmd_receiver) = channel::<ConsoleCommand>();

    let wireless_connected = Arc::new(AtomicBool::new(false));
    let wireless_connected_read = wireless_connected.clone();

    // Spawn serial console command task
    thread::spawn(move || {
        let mut read_buf: [u8; 64] = [0; 64];
        loop {
            // TODO: Clean up
            match std::io::stdin().read(&mut read_buf) {
                Ok(n) => {
                    let buf = &read_buf[..n];

                    if n != 0 && buf[0] != b'\n' {
                        if let Some(cmd) = console::parse(buf) {
                            console_cmd_sender.send(cmd).expect("Failed to send console command");
                        }
                    }
                }
                Err(_) => {}
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Control task
    //
    // 1. Get sensor input
    // 2. Pass sensor data to the state estimator
    // 3. Use estimated state in PID control loop
    // 4. 'Mix' motor output
    thread::spawn(move || {
        let offsets = calibrate_imu(500, 20, || {
            let a = imu.get_acc().unwrap();
            let g = imu.get_gyro().unwrap();

            ((a.x, a.y, a.z), (g.x, g.y, g.z))
        });

        let mut estimator = StateEstimator::default();
        let mut last_measurement = Instant::now();

        loop {
            // Read IMU data
            let accel = imu.get_acc();
            let gyro = imu.get_gyro();
            let temp = imu.get_temp();

            let now = Instant::now();
            let delta_time = now.duration_since(last_measurement).as_secs_f32();
            last_measurement = now;

            if let (Ok(accel), Ok(gyro), Ok(_temp)) = (accel, gyro, temp) {
                let accel = AccelerometerData {
                    x: accel.x - offsets.ax_offset,
                    y: accel.y - offsets.ay_offset,
                    z: accel.z - offsets.az_offset,
                };
                let gyro = GyroscopeData {
                    x: gyro.x - offsets.gx_offset,
                    y: gyro.y - offsets.gy_offset,
                    z: gyro.z - offsets.gz_offset,
                };

                let input = EstimatorInput {
                    accel,
                    gyro,
                    altitude: 0.0,
                };
                state_sender
                    .send(IcarusState::Sensors(input.clone()))
                    .expect("Failed to send input");

                if let Ok(estimated_state) = estimator.update(input, delta_time) {
                    state_sender
                        .send(IcarusState::EstimatedState(estimated_state))
                        .expect("Failed to send state");
                }
            }

            thread::sleep(Duration::from_millis(20));
        }
    });

    // Spawn LED task
    thread::spawn(move || {
        let mut wireless_connected = false;

        loop {
            let is_connected = wireless_connected_read.load(Ordering::Relaxed);

            let (color, duration) = if is_connected {
                (StatColor::Green, 1000)
            } else {
                (StatColor::Red, 300)
            };

            stat_led.update(color).unwrap();
            thread::sleep(Duration::from_millis(duration));

            stat_led.update(StatColor::Black).unwrap();
            thread::sleep(Duration::from_millis(duration));
        }
    });

    defmt::info!("Starting IDLE task");

    let mut send_buf: [u8; 128] = [0; 128];

    // Idle Task
    loop {
        // Write all recieved state to serial port
        for state in state_reciever.try_iter() {
            let used_buf = icarus_wire::encode(&state, &mut send_buf)?;
            //std::io::stdout().write_all(&used_buf)?;
            // std::io::stdout().flush()?;
            // TODO(nnarain): Why is this needed?
            //print!("\n");
        }

        // Recieve commands and dispatch to tasks
        for cmd in cmd_reciever.try_iter() {
            match cmd {
                IcarusCommand::CycleLed => {}
            }
        }

        for cmd in console_cmd_receiver.try_iter() {
            match cmd {
                ConsoleCommand::Wireless(cmd) => {
                    match cmd {
                        WirelessCommands::Get => print_wifi_settings(&mut wifi)?,
                        _ => {}
                    }
                }
            }
        }

        let connected = wireless_connected.load(Ordering::Relaxed);
        if !connected {
            // Check if wifi is connected
            let is_connected = wifi.is_connected().unwrap_or(false);
            wireless_connected.store(is_connected, Ordering::Relaxed);
        }

        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}

fn print_wifi_settings(wifi: &mut AppWifi) -> anyhow::Result<()> {
    let connected = wifi.is_connected().unwrap_or(false);
    if connected {
        let ip_settings = wifi.get_ip_settings()?;
        println!("{:?}", ip_settings);
    }
    else {
        println!("Not connected");
    }

    Ok(())
}

/// Sample accelerometer and gyro data and calculate the device specific offset
fn calibrate_imu<F>(samples: usize, delay_ms: u64, mut f: F) -> ImuCalibrationOffset
where
    F: FnMut() -> ((f32, f32, f32), (f32, f32, f32)),
{
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
