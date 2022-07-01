//
// user_button.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 01 2022
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
    let btn = hw.user_btn;
    let mut delay = hw.delay;

    let error_color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };

    let off_color = Hsv {
        hue: 50,
        sat: 255,
        val: 255,
    };

    let on_color = Hsv {
        hue: 150,
        sat: 255,
        val: 255,
    };

    let mut data;

    loop {
        let color = {
            match btn.is_low() {
                Ok(state) => {
                    if state {
                        off_color
                    } else {
                        on_color
                    }
                }
                Err(_) => error_color,
            }
        };

        data = [hsv2rgb(color)];

        led.write(brightness(gamma(data.iter().cloned()), 10))
            .unwrap();
        delay.delay_ms(20u8);
    }
}
