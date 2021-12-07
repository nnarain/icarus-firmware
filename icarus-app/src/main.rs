//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 06 2021
//
#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = icarus::hal::pac, peripherals = true, dispatchers = [EXTI3, EXTI4])]
mod app {
    use icarus::{
        prelude::*,
        cortex_m,
        hal::pac,
        types::{PinStat1, PinStat2},
    };

    #[shared]
    struct Shared{}

    #[local]
    struct Local {
        stat1: PinStat1,
        stat2: PinStat2,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Get peripherals
        let cp: cortex_m::Peripherals = cx.core;
        let dp: pac::Peripherals = cx.device;

        // Initialize hardware
        let hw = Icarus::new(cp, dp).unwrap();
        let stat1 = hw.stat1;
        let stat2 = hw.stat2;

        (
            Shared{},
            Local{
                stat1,
                stat2,
            },
            init::Monotonics()
        )
    }

    #[idle(local = [stat1, stat2])]
    fn idle(cx: idle::Context) -> ! {
        let stat1 = cx.local.stat1;
        let stat2 = cx.local.stat2;

        stat2.set_high().unwrap();

        loop {
            stat1.set_high().unwrap();
            stat2.set_low().unwrap();

            cortex_m::asm::delay(8_000_000);

            stat1.set_low().unwrap();
            stat2.set_high().unwrap();

            cortex_m::asm::delay(8_000_000);
        }
    }
}
