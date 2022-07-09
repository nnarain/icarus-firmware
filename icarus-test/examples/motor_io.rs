//
// motors_io.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 08 2022
//
#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;

use icarus::{prelude::*, Icarus};

#[entry]
fn main() -> ! {
    let hw = Icarus::take().unwrap();

    let mut drv1_en = hw.drv1_en;
    let mut drv2_en = hw.drv2_en;

    let mut rtrctl1 = hw.rtrctl1;
    let mut rtrctl2 = hw.rtrctl2;
    let mut rtrctl3 = hw.rtrctl3;
    let mut rtrctl4 = hw.rtrctl4;

    let mut delay = hw.delay;

    drv1_en.set_high().unwrap();
    drv2_en.set_high().unwrap();

    loop {
        rtrctl1.toggle().unwrap();
        rtrctl2.toggle().unwrap();
        rtrctl3.toggle().unwrap();
        rtrctl4.toggle().unwrap();

        delay.delay_ms(1000u32);
    }
}
