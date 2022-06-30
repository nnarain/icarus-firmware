//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 28 2021
//
#![no_std]

pub use esp32c3_hal as hal;

pub mod prelude {
    pub use crate::hal::prelude::*;
}

use crate::hal::{
    clock::ClockControl,
    gpio::*,
    gpio_types::{Output, PushPull},
    pac::Peripherals,
    prelude::*,
    Delay, RtcCntl, Timer,
};

#[derive(Debug)]
pub enum IcarusError {
    HardwareInitError,
}

/// Icarus Hardware Interface
pub struct Icarus {
    pub led: Gpio10<Output<PushPull>>,
    pub delay: Delay,
}

impl Icarus {
    pub fn take() -> Result<Icarus, IcarusError> {
        if let Some(dp) = Peripherals::take() {
            let system = dp.SYSTEM.split();
            let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

            // Disable watchdog timers
            let mut rtc_cntl = RtcCntl::new(dp.RTC_CNTL);
            let mut timer0 = Timer::new(dp.TIMG0);
            let mut timer1 = Timer::new(dp.TIMG1);

            rtc_cntl.set_super_wdt_enable(false);
            rtc_cntl.set_wdt_enable(false);
            timer0.disable();
            timer1.disable();

            let io = IO::new(dp.GPIO, dp.IO_MUX);

            let led = io.pins.gpio10.into_push_pull_output();

            let delay = Delay::new(&clocks);

            Ok(Icarus { led, delay })
        } else {
            Err(IcarusError::HardwareInitError)
        }
    }
}
