//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 06 2021
//
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use panic_halt as _;

use embassy::{
    executor::Spawner,
    time::{Duration, Timer},
};

use embassy_esp::pac::Peripherals;

use icarus::{
    hal::{
        gpio::Gpio10,
        gpio_types::{Output, PushPull},
    },
    prelude::*,
    Icarus,
};

#[embassy::task]
async fn pong(mut led: Gpio10<Output<PushPull>>) {
    loop {
        led.toggle().unwrap();
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy::main]
async fn main(spawner: Spawner, p: Peripherals) {
    let hw = Icarus::init(p).unwrap();

    let led = hw.drv1_en;

    spawner.spawn(pong(led)).unwrap();

    loop {
        Timer::after(Duration::from_millis(100)).await;
    }
}
