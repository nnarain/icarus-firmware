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

    let mut stat1 = icarus.stat1;
    let mut stat2 = icarus.stat2;

    stat2.toggle().unwrap();

    loop {
        stat1.toggle().unwrap();
        stat2.toggle().unwrap();
        asm::delay(8_000_000);
    }
}
