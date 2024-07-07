#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    peripherals,
    usart::{self, UartTx, Config as UartConfig},
};

use panic_halt as _;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let dp = embassy_stm32::init(Default::default());

    // Use default configuration for UART: 115200 baud
    let usart_config = UartConfig::default();

    let mut uart_tx = UartTx::new(dp.USART1, dp.PA9, dp.DMA2_CH7, usart_config).unwrap();

    loop {
        // uart_tx.blocking_write("Hello World\r\n".as_bytes()).unwrap();
        uart_tx.write("Hello Async World!\r\n".as_bytes()).await.unwrap();
    }
}
