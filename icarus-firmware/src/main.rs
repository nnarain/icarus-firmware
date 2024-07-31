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
    bind_interrupts,
    gpio::{Level, Output, Speed},
    peripherals,
    usart::{self, Config as UartConfig, Uart, UartRx, UartTx},
    i2c::{self, I2c},
    time::Hertz,
};
use embassy_time::Timer;
use embassy_sync::channel::Channel;

use panic_halt as _;

use icarus_firmware::{
    rc::RcInputDecoder,
    telemetry::Telemetry,
    sensors::{EstimatedState, Attitude},
    queues::*,
};

use mpu6050_dmp::{
    sensor_async::Mpu6050,
    address::Address as Mpu6050Address,
};
use ahrs::{Ahrs, Madgwick};
use nalgebra::Vector3;

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
    let led = Output::new(dp.PB0, Level::Low, Speed::Low);

    // Serial hardware
    // Use default configuration for UART: 115200 baud
    let usart_config = UartConfig::default();

    let (uart_tx, uart_rx) = Uart::new(dp.USART1, dp.PA10, dp.PA9, Irqs, dp.DMA2_CH7, dp.DMA2_CH2, usart_config)
                                    .unwrap().split();

    // I2C hardware
    let i2c = I2c::new(dp.I2C1, dp.PB6, dp.PB7, Irqs, dp.DMA1_CH6, dp.DMA1_CH0, Hertz(400_000), Default::default());

    let imu = Mpu6050::new(i2c, Mpu6050Address::default()).await.unwrap();

    // Spawn tasks

    // RC input task
    spawner.spawn(rc_input_task(uart_rx, RC_INPUT_CHNL.sender())).unwrap();
    // Sensors task
    spawner.spawn(sensors_task(imu, ESTIMATED_STATE_CHNL.sender())).unwrap();
    // Control loop task
    // TODO(nnarain): PWM
    spawner.spawn(control_task(RC_INPUT_CHNL.receiver(), ESTIMATED_STATE_CHNL.receiver(), TELEMETRY_CHNL.sender())).unwrap();
    // LED task
    spawner.spawn(led_task(led)).unwrap();
    // Telemetry task
    spawner.spawn(telemetry_task(uart_tx, TELEMETRY_CHNL.receiver())).unwrap();
}

#[embassy_executor::task]
async fn control_task(rc_input: RcInputChannelReceiver, estimated_state: EstimatedStateChannelReceiver, telemetry: TelemetryChannelSender) {
    loop {
        // TODO(nnarain): this needs to timeout
        let input = rc_input.receive().await;
        let state = estimated_state.receive().await;

        let telemetry_msg = Telemetry {chnl0: input.chnl0};
        telemetry.send(telemetry_msg).await;
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
async fn telemetry_task(mut uart: UartTx<'static, peripherals::USART1, peripherals::DMA2_CH7>, telemetry: TelemetryChannelReceiver) {
    loop {
        let telemetry = telemetry.receive().await;

        let b0 = (telemetry.chnl0 & 0x0F) as u8;
        let b1 = (telemetry.chnl0 >> 8) as u8;
        let buf: [u8; 0x02] = [b0, b1];

        uart.write(&buf[..]).await.unwrap();
    }
}
