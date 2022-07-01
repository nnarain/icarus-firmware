//
// stat_led.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 01 2022
//
#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;

use icarus::{prelude::*, Icarus};

#[entry]
fn main() -> ! {
    let hw = Icarus::take().unwrap();
    let mut led = hw.stat;
    let delay = hw.delay;

    loop {
        led.toggle().unwrap();
        delay.delay(1000_000u32);
    }
}
