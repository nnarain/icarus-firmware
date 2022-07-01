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
    // TODO: I2C

    // Drive 2 Enable
    pub drv2_en: Gpio6<Output<PushPull>>,
    // Rotor 2 Control
    pub rtrctl2: Gpio7<Output<PushPull>>,
    // Robot 1 Control
    pub rtrctl1: Gpio8<Output<PushPull>>,

    // Drive 1 Enable
    pub drv1_en: Gpio10<Output<PushPull>>,
    // Rotor 4 Control
    pub rtrctl4: Gpio4<Output<PushPull>>,
    // Rotor 3 Control
    pub rtrctl3: Gpio5<Output<PushPull>>,

    // Status LED
    pub stat: Gpio21<Output<PushPull>>,

    // Battery sense
    pub battery_sense: Gpio3<Output<PushPull>>,

    // Delay
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

            let drv2_en = io.pins.gpio6.into_push_pull_output();
            let rtrctl2 = io.pins.gpio7.into_push_pull_output();
            let rtrctl1 = io.pins.gpio8.into_push_pull_output();

            let drv1_en = io.pins.gpio10.into_push_pull_output();
            let rtrctl4 = io.pins.gpio4.into_push_pull_output();
            let rtrctl3 = io.pins.gpio5.into_push_pull_output();

            let stat = io.pins.gpio21.into_push_pull_output();

            let battery_sense = io.pins.gpio3.into_push_pull_output();

            let delay = Delay::new(&clocks);

            Ok(Icarus {
                drv1_en,
                drv2_en,
                rtrctl1,
                rtrctl2,
                rtrctl3,
                rtrctl4,
                stat,
                battery_sense,
                delay,
            })
        } else {
            Err(IcarusError::HardwareInitError)
        }
    }
}
