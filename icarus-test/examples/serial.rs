//
// serial.rs
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

use core::fmt::Write;

#[entry]
fn main() -> ! {
    let icarus = Icarus::new().unwrap();
    let mut usart1 = icarus.usart1;
    let mut usart2 = icarus.usart2;

    let mut stat1 = icarus.stat1;

    loop {
        // Show activity
        stat1.toggle().unwrap();

        write!(usart1, "Hello USART1!\r\n").unwrap();
        write!(usart2, "Hello USART2!\r\n").unwrap();

        asm::delay(8_000_000);
    }
}
