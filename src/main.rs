#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::usart::{BufferedUart, Config, Parity, StopBits, DataBits};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embedded_io_async::BufRead;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::BufferedInterruptHandler<peripherals::USART1>;
});

enum State {
    Header,
    Channels,
    Footer
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // The `Config` struct is marked as non_exhaustive
    let mut usart_config = Config::default();
    usart_config.baudrate = 115200;
    // usart_config.parity = Parity::ParityEven;
    // usart_config.stop_bits = StopBits::STOP2;

    let mut tx_buf = [0u8; 32];
    let mut rx_buf = [0u8; 32];
    let mut buf_usart = BufferedUart::new(p.USART1, Irqs, p.PA10, p.PA9, &mut tx_buf, &mut rx_buf, usart_config).unwrap();

    let mut led = Output::new(p.PB0, Level::Low, Speed::Low);

    // let mut state = State::Header;
    // let mut channel_data = [0u8; 22];
    // let mut recv_channel_bytes = 0;

    let mut mybuf = [0u8; 10];
    let mut count = 0;

    loop {
        let buf = buf_usart.fill_buf().await.unwrap();
        // info!("{}", buf);

        // for &b in buf {
        //     info!("b: {}", b);
        //     state = match state {
        //         State::Header => {
        //             if b == 0x0F {
        //                 State::Channels
        //             }
        //             else {
        //                 State::Header
        //             }
        //         },
        //         State::Channels => {
        //             channel_data[recv_channel_bytes] = b;
        //             recv_channel_bytes += 1;

        //             if recv_channel_bytes == 22 {
        //                 recv_channel_bytes = 0;
        //                 State::Footer
        //             }
        //             else {
        //                 State::Footer
        //             }
        //         },
        //         State::Footer => {
        //             led.set_high();
        //             State::Header
        //         },
        //     }
        // }

        for (i, &b) in buf.iter().enumerate() {
            mybuf[count + i] = b;
        }

        // Read bytes have to be explicitly consumed, otherwise fill_buf() will return them again
        let n = buf.len();
        count += n;

        if count >= 5 {
            info!("count: {}, mybuf: {}", count, mybuf);
            led.set_high();
        }

        buf_usart.consume(n);
    }
}
