#![no_std]
#![no_main]

// https://docs.embassy.dev/embassy-stm32/git/stm32f405rg/index.html

use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    peripherals,
    gpio::{Level, Output, Speed},
    usart::{self, Uart, BufferedUart, Config as UartConfig},
};
use embedded_io_async::{BufRead, Write};
use panic_halt as _;

// bind_interrupts!(struct Irqs {
//     USART1 => usart::BufferedInterruptHandler<peripherals::USART1>;
// });

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let dp = embassy_stm32::init(Default::default());

    let mut led = Output::new(dp.PB0, Level::Low, Speed::Low);
    led.set_low();

    // Use default configuration for UART: 115200 baud
    let usart_config = UartConfig::default();

    // let mut uart_tx = UartTx::new(dp.USART1, dp.PA9, dp.DMA2_CH7, usart_config).unwrap();
    // let mut uart_rx = UartRx::new(dp.USART1, Irqs, dp.PA10, NoDma, usart_config).unwrap();

    let mut rx_buf = [0u8; 32];
    let (mut uart_tx, mut uart_rx) = Uart::new(dp.USART1, dp.PA10, dp.PA9, Irqs, dp.DMA2_CH7, dp.DMA2_CH2, usart_config)
                                    .unwrap().split();

    loop {
        let len = uart_rx.read_until_idle(&mut rx_buf).await.unwrap();
        if len > 0 {
            if rx_buf[0] == 65 {
                led.set_high();
            }
        }
    }

    // loop {


    //     // uart_tx.blocking_write("Hello World\r\n".as_bytes()).unwrap();
    //     uart_tx.write("Hello Async World!\r\n".as_bytes()).await.unwrap();
    // }

    // let mut tx_buf = [0u8; 32];
    // let mut rx_buf = [0u8; 32];

    // let (mut usart_tx, mut usart_rx) = BufferedUart::new(dp.USART1, Irqs, dp.PA10, dp.PA9, &mut tx_buf, &mut rx_buf, usart_config).unwrap().split();

    // let mut other_buf = [0u8; 32];

    // loop {
    //     // let buf = usart_rx.fill_buf().await.unwrap();
    //     // other_buf.copy_from_slice(buf);

    //     // let n = buf.len();
    //     // usart_rx.consume(n);

    //     usart_tx.write_all("Hello\r\n".as_bytes()).await.unwrap();
    // }
}
