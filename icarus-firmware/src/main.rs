#![no_std]
#![no_main]

// https://docs.embassy.dev/embassy-stm32/git/stm32f405rg/index.html

use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, gpio::{Level, Output, Speed}, peripherals, usart::{self, Config as UartConfig, Uart, UartRx}
};
use embassy_time::Timer;
use embassy_sync::channel::Channel;

use panic_halt as _;

use icarus_firmware::{
    rc::RcInputDecoder,
    queues::{
        RcInputChannel, RcInputChannelSender, RcInputChannelReceiver,
        SensorStateChannel, SensorStateChannelSender, SensorStateChannelReceiver,
    }
};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

// Channel used to receive RC data from UART
static RC_INPUT_CHNL: RcInputChannel = Channel::new();
// Channel used to receive sensor data
static SENSOR_STATE_CHNL: SensorStateChannel = Channel::new();

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

    let (_uart_tx, uart_rx) = Uart::new(dp.USART1, dp.PA10, dp.PA9, Irqs, dp.DMA2_CH7, dp.DMA2_CH2, usart_config)
                                    .unwrap().split();


    // Spawn tasks

    // RC input task
    spawner.spawn(rc_input_task(uart_rx, RC_INPUT_CHNL.sender())).unwrap();
    // Control loop task
    // TODO(nnarain): PWM
    spawner.spawn(control_task(RC_INPUT_CHNL.receiver(), SENSOR_STATE_CHNL.receiver())).unwrap();
    // LED task
    spawner.spawn(led_task(led)).unwrap();
}

#[embassy_executor::task]
async fn control_task(rc_input: RcInputChannelReceiver, sensor_state: SensorStateChannelReceiver) {
    loop {
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::task]
async fn sensors_task() {
    loop {
        Timer::after_millis(1000).await;
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
