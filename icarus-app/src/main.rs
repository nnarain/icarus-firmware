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
        hal::{
            serial::Event,
            Toggle,
        },
        types::{PinStat1, PinStat2, Serial1},
    };

    #[shared]
    struct Shared{}

    #[local]
    struct Local {
        stat1: PinStat1,
        stat2: PinStat2,

        serial1: Serial1,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Initialize hardware
        let hw = Icarus::new(cx.core, cx.device).unwrap();

        // LED indicators
        let stat1 = hw.stat1;
        let stat2 = hw.stat2;

        // Serial port 1. Configure interrupt for data receive
        let mut serial1 = hw.usart1;
        serial1.configure_interrupt(Event::ReceiveDataRegisterNotEmpty, Toggle::On);

        (
            Shared{},
            Local{
                stat1,
                stat2,

                serial1,
            },
            init::Monotonics()
        )
    }

    #[task(binds = USART1_EXTI25, local = [serial1])]
    fn serial_task(cx: serial_task::Context) {
        let serial = cx.local.serial1;

        if serial.is_event_triggered(Event::ReceiveDataRegisterNotEmpty) {
            if let Ok(byte) = serial.read() {
                serial.write(byte).unwrap();
            }
        }
    }

    ///
    /// Show activity by flashing the STAT LEDs
    /// 
    #[idle(local = [stat1, stat2])]
    fn idle(cx: idle::Context) -> ! {
        let stat1 = cx.local.stat1;
        let stat2 = cx.local.stat2;

        stat2.set_high().unwrap();

        loop {
            stat1.toggle().unwrap();
            stat2.toggle().unwrap();

            // TODO: Use `delay` here
            cortex_m::asm::delay(8_000_000);
        }
    }
}
