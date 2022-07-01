//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 06 2021
//
#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;

use icarus::{
    prelude::*,
    smart_leds::{
        brightness, gamma,
        hsv::{hsv2rgb, Hsv},
        SmartLedsWrite,
    },
    Icarus,
};

#[entry]
fn main() -> ! {
    let hw = Icarus::take().unwrap();
    let mut led = hw.stat;
    let mut delay = hw.delay;

    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };
    let mut data;

    loop {
        for hue in 0..=255 {
            color.hue = hue;
            data = [hsv2rgb(color)];

            led.write(brightness(gamma(data.iter().cloned()), 1))
                .unwrap();

            delay.delay_ms(20u8);
        }
    }
}
