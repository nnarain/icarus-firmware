//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 16 2024
//

#![no_std]
#![no_main]

// https://docs.embassy.dev/embassy-stm32/git/stm32f405rg/index.html

use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, gpio::{Output, OutputType}, i2c::{self, I2c}, peripherals, time::{hz, Hertz}, timer::{simple_pwm::{PwmPin, SimplePwm}, Channel as PwmChannel}, usart::{self, Config as UartConfig, Uart, UartRx, UartTx}
};
use embassy_time::{with_timeout, Duration, Timer};
use embassy_sync::channel::Channel;

use panic_halt as _;

use icarus_firmware::{
    rc::RcInputDecoder,
    telemetry::Telemetry,
    sensors::{EstimatedState, Attitude},
    queues::*,
    MAX_ROTOR_THROTTLE, MIN_ROTOR_THROTTLE, ROTOR_PWM_FREQ
};

use mpu6050_dmp::{
    sensor_async::Mpu6050,
    address::Address as Mpu6050Address,
};
use ahrs::{Ahrs, Madgwick};
use nalgebra::Vector3;
use pid::Pid;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;

    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

// Channel used to receive RC data from UART
static RC_INPUT_CHNL: RcInputChannel = Channel::new();
// Channel used to receive sensor data
static ESTIMATED_STATE_CHNL: EstimatedStateChannel = Channel::new();
// Channel used to communicate telemetry data to a host system
static TELEMETRY_CHNL: TelemetryChannel = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Configure device core and get peripherals
    let dp = embassy_stm32::init(Default::default());

    // Hardware Setup

    // LED hardware
    // let _led = Output::new(dp.PB0, Level::Low, Speed::Low);
    // let mut spi_config = spi::Config::default();
    // spi_config.frequency = khz(12_800);

    // let spi = Spi::new_txonly_nosck(dp.SPI1, dp.PB0, dp.DMA1_CH0, spi_config);

    // Serial hardware
    // Use default configuration for UART: 115200 baud
    let usart_config = UartConfig::default();

    let (uart_tx, uart_rx) = Uart::new(dp.USART1, dp.PA10, dp.PA9, Irqs, dp.DMA2_CH7, dp.DMA2_CH2, usart_config)
                                    .unwrap().split();

    // I2C hardware
    let i2c = I2c::new(dp.I2C1, dp.PB6, dp.PB7, Irqs, dp.DMA1_CH6, dp.DMA1_CH0, Hertz(400_000), Default::default());

    let imu = Mpu6050::new(i2c, Mpu6050Address::default()).await.unwrap();

    // PWM hardware
    let ch1 = PwmPin::new_ch1(dp.PA0, OutputType::PushPull);
    let ch2 = PwmPin::new_ch2(dp.PA1, OutputType::PushPull);
    let ch3 = PwmPin::new_ch3(dp.PA2, OutputType::PushPull);
    let ch4 = PwmPin::new_ch4(dp.PA3, OutputType::PushPull);

    let pwm = SimplePwm::new(dp.TIM5, Some(ch1), Some(ch2), Some(ch3), Some(ch4), hz(ROTOR_PWM_FREQ), Default::default());

    // Spawn tasks

    // RC input task
    spawner.spawn(rc_input_task(uart_rx, RC_INPUT_CHNL.sender())).unwrap();
    // Sensors task
    spawner.spawn(sensors_task(imu, ESTIMATED_STATE_CHNL.sender())).unwrap();
    // Control loop task
    // TODO(nnarain): PWM
    spawner.spawn(control_task(RC_INPUT_CHNL.receiver(), ESTIMATED_STATE_CHNL.receiver(), pwm, TELEMETRY_CHNL.sender())).unwrap();
    // LED task
    //spawner.spawn(led_task(led)).unwrap();
    // Telemetry task
    spawner.spawn(telemetry_task(uart_tx, TELEMETRY_CHNL.receiver())).unwrap();
}

#[embassy_executor::task]
async fn control_task(rc_input: RcInputChannelReceiver, estimated_state: EstimatedStateChannelReceiver, mut pwm: SimplePwm<'static, peripherals::TIM5>, _telemetry: TelemetryChannelSender) {
    // PID controller for pitch
    let mut pitch_pid: Pid<f32> = Pid::new(0.0, 10.0);
    pitch_pid.p(8.75, 100.0);
    pitch_pid.i(3.5, 100.0);
    pitch_pid.d(0.1, 100.0);

    // PID controller roll
    let mut roll_pid: Pid<f32> = Pid::new(0.0, 10.0);
    roll_pid.p(1.0, 100.0);
    roll_pid.i(3.5, 100.0);
    roll_pid.d(0.1, 100.0);

    // PID controller for yaw
    let mut yaw_pid: Pid<f32> = Pid::new(0.0, 10.0);
    yaw_pid.p(1.0, 100.0);
    yaw_pid.i(3.5, 100.0);
    yaw_pid.d(0.1, 100.0);

    // Enable all PWM outputs
    pwm.enable(PwmChannel::Ch1);
    pwm.enable(PwmChannel::Ch2);
    pwm.enable(PwmChannel::Ch3);
    pwm.enable(PwmChannel::Ch4);

    loop {
        // Get the RC input and estimated state

        // TODO(nnarain): this needs to timeout
        let input = with_timeout(Duration::from_millis(10), rc_input.receive()).await.unwrap_or_default();
        let state = estimated_state.receive().await;

        // Pitch, roll, yaw input from RC controller
        let (pitch_input, roll_input, yaw_input, throttle) = input.throttle();
        // Estimated pitch, roll, yaw
        let Attitude {pitch, roll, yaw} = state.attitude;

        // Update the PID controllers with the new set points
        pitch_pid.setpoint(pitch_input);
        roll_pid.setpoint(roll_input);
        yaw_pid.setpoint(yaw_input);

        // Get the output of the PID controller
        let pitch_output = pitch_pid.next_control_output(pitch).output;
        let roll_output = roll_pid.next_control_output(roll).output;
        let yaw_output = yaw_pid.next_control_output(yaw).output;

        // Mix the PID outputs to get the individual rotor throttles

        /*
          Rotor Layout

             ^^
          (4)  (2)
             \/
             /\
          (3)  (1)
        */
        let t1 = throttle + pitch_output + roll_output - yaw_output;
        let t2 = throttle - pitch_output + roll_output + yaw_output;
        let t3 = throttle + pitch_output - roll_output + yaw_output;
        let t4 = throttle - pitch_output - roll_output - yaw_output;

        let t1 = (t1 as u16).clamp(MIN_ROTOR_THROTTLE, MAX_ROTOR_THROTTLE);
        let t2 = (t2 as u16).clamp(MIN_ROTOR_THROTTLE, MAX_ROTOR_THROTTLE);
        let t3 = (t3 as u16).clamp(MIN_ROTOR_THROTTLE, MAX_ROTOR_THROTTLE);
        let t4 = (t4 as u16).clamp(MIN_ROTOR_THROTTLE, MAX_ROTOR_THROTTLE);

        pwm.set_duty(PwmChannel::Ch1, t1);
        pwm.set_duty(PwmChannel::Ch2, t2);
        pwm.set_duty(PwmChannel::Ch3, t3);
        pwm.set_duty(PwmChannel::Ch4, t4);

        let _telemetry_msg = Telemetry {state,};
        // telemetry.send(telemetry_msg).await;
    }
}

#[embassy_executor::task]
async fn sensors_task(mut imu: Mpu6050<I2c<'static, peripherals::I2C1, peripherals::DMA1_CH6, peripherals::DMA1_CH0>>, estimated_state: EstimatedStateChannelSender) {
    let mut ahrs = Madgwick::default();

    loop {
        let accel = imu.accel().await.unwrap();
        let gyro = imu.gyro().await.unwrap();

        // TODO(nnarain): Check units
        let accel = Vector3::new(accel.x() as f64, accel.y() as f64, accel.z() as f64);
        let gyro = Vector3::new(gyro.x() as f64, gyro.y() as f64, gyro.z() as f64);

        let quat = ahrs.update_imu(&gyro, &accel).unwrap();

        let (roll, pitch, yaw) = quat.euler_angles();
        let (roll, pitch, yaw) = (roll as f32, pitch as f32, yaw as f32);

        let attitude = Attitude { pitch, roll, yaw };
        let state = EstimatedState { attitude, };

        estimated_state.send(state).await;
    }
}

#[embassy_executor::task]
async fn rc_input_task(uart: UartRx<'static, peripherals::USART1, peripherals::DMA2_CH2>, rc_input: RcInputChannelSender) {
    // UART RX DMA buffer
    let mut dma_buf = [0u8; 64];

    // Create a buffered uart instance
    let mut uart = uart.into_ring_buffered(&mut dma_buf[..]);
    // Buffer for receiving inbound data
    let mut rx_buf = [0u8; 64];

    // Decoder for incoming byte stream
    let mut decoder = RcInputDecoder::default();

    loop {
        // Read some data from UART
        let num_bytes = uart.read(&mut rx_buf[..]).await.unwrap();

        // Update the decoder with the incoming data and send the rc input through the channel when it's received.
        for byte in &rx_buf[..num_bytes] {
            if let Some(rc) = decoder.update(*byte) {
                rc_input.send(rc).await;
            }
        }
    }
}

// TODO(nnarain): This is on the devboard not icarus.
#[embassy_executor::task]
async fn led_task(mut led: Output<'static, peripherals::PB0>) {
    // Toggle at 1Hz
    loop {
        led.toggle();
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::task]
async fn telemetry_task(mut _uart: UartTx<'static, peripherals::USART1, peripherals::DMA2_CH7>, telemetry: TelemetryChannelReceiver) {
    loop {
        let telemetry = telemetry.receive().await;
        let _state = telemetry.state;

        // TODO(nnarain): This is blocking and needs to be updated
        // write!(uart, "Pitch: {}, Roll: {}, Yaw: {}\r\n", state.attitude.pitch, state.attitude.roll, state.attitude.yaw).unwrap();
    }
}
