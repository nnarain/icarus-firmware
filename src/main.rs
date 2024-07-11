#![no_std]
#![no_main]

// https://docs.embassy.dev/embassy-stm32/git/stm32f405rg/index.html

use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, gpio::{Level, Output, Speed}, peripherals, usart::{self, Config as UartConfig, Uart, UartRx}
};
use embassy_time::Timer;
use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::{Channel, Sender, Receiver}
};
use panic_halt as _;

struct RcThrottle {
    pub forwards: u16,
}

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

// Channel use to receive RC data from UART
static RC_INPUT_CHNL: Channel<ThreadModeRawMutex, RcThrottle, 3> = Channel::new();

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
    // LED task
    spawner.spawn(led_task(led, RC_INPUT_CHNL.receiver())).unwrap();
}

#[embassy_executor::task]
async fn control_task() {
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
async fn rc_input_task(mut uart: UartRx<'static, peripherals::USART1, peripherals::DMA2_CH2>, chnl: Sender<'static, ThreadModeRawMutex, RcThrottle, 3>) {
    let mut buf = [0u8; 32];
    loop {
        let len = uart.read_until_idle(&mut buf).await.unwrap();
        if len > 0 {
            if buf[0] == 65 {
                let throttle = RcThrottle{forwards: 1};
                chnl.send(throttle).await;
            }
            else if buf[0] == 66 {
                let throttle = RcThrottle{forwards: 2};
                chnl.send(throttle).await;
            }
        }
    }
}

// TODO(nnarain): This is on the devboard not icarus.
#[embassy_executor::task]
async fn led_task(mut led: Output<'static, peripherals::PB0>, chnl: Receiver<'static, ThreadModeRawMutex, RcThrottle, 3>) {
    // Toggle at 1Hz
    // loop {
    //     led.toggle();
    //     Timer::after_millis(1000).await;
    // }
    loop {
        let throttle = chnl.receive().await;
        if throttle.forwards == 1 {
            led.set_high();
        }
        else if throttle.forwards == 2 {
            led.set_low();
        }
    }
}
