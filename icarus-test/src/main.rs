
//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 27 2021
//
#![no_std]
#![no_main]

use panic_halt as _;

use icarus::{
    entry,
    prelude::*,
    hal::pac,
    cortex_m::asm,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut stat1 = gpioa
        .pa4
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut stat2 = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    stat2.toggle().unwrap();

    loop {
        stat1.toggle().unwrap();
        stat2.toggle().unwrap();
        asm::delay(8_000_000);
    }
}
