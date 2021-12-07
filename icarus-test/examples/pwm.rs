//
// blink.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 29 2021
//
#![no_std]
#![no_main]

use panic_halt as _;

use icarus::{
    entry,
    prelude::*,
    cortex_m::asm,
};

#[entry]
fn main() -> ! {
    let icarus = Icarus::take().unwrap();

    let mut pwm1 = icarus.pwm1;
    let mut pwm2 = icarus.pwm2;
    let mut pwm3 = icarus.pwm3;
    let mut pwm4 = icarus.pwm4;
    let mut pwm6 = icarus.pwm6;

    pwm1.enable();
    pwm2.enable();
    pwm3.enable();
    pwm4.enable();
    pwm6.enable();

    let max_duty = pwm1.get_max_duty() as u32;

    loop {
        for duty in 0..max_duty {
            pwm1.set_duty(duty);
            pwm2.set_duty(duty);
            pwm3.set_duty(duty);
            pwm4.set_duty(duty);
            pwm6.set_duty(duty as u16);

            asm::delay(8_000);
        }
    }
}
