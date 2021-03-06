//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 28 2021
//
#![no_std]

pub use esp32c3_hal as hal;
use hal::gpio_types::Input;
pub use smart_leds;

pub mod prelude {
    pub use crate::hal::prelude::*;
}

use crate::hal::{
    clock::ClockControl,
    gpio::*,
    gpio_types::{Floating, Output, PushPull, Unknown},
    pac::Peripherals,
    prelude::*,
    pulse_control::{Channel0, ClockSource},
    utils::{smartLedAdapter, SmartLedsAdapter},
    Delay, PulseControl, RtcCntl, Timer,
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

    // User Button
    pub user_btn: Gpio9<Input<Floating>>,

    // Status LED
    pub stat: SmartLedsAdapter<Channel0, Gpio21<Unknown>, 25>,

    // Battery sense
    pub battery_sense: Gpio3<Output<PushPull>>,

    // Delay
    pub delay: Delay,
}

impl Icarus {
    pub fn init(p: Peripherals) -> Result<Icarus, IcarusError> {
        let mut system = p.SYSTEM.split();
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

        // Disable watchdog timers
        let mut rtc_cntl = RtcCntl::new(p.RTC_CNTL);
        let mut timer0 = Timer::new(p.TIMG0);
        let mut timer1 = Timer::new(p.TIMG1);

        rtc_cntl.set_super_wdt_enable(false);
        rtc_cntl.set_wdt_enable(false);
        timer0.disable();
        timer1.disable();

        let io = IO::new(p.GPIO, p.IO_MUX);

        let drv2_en = io.pins.gpio6.into_push_pull_output();
        let rtrctl2 = io.pins.gpio7.into_push_pull_output();
        let rtrctl1 = io.pins.gpio8.into_push_pull_output();

        let drv1_en = io.pins.gpio10.into_push_pull_output();
        let rtrctl4 = io.pins.gpio4.into_push_pull_output();
        let rtrctl3 = io.pins.gpio5.into_push_pull_output();

        let user_btn = io.pins.gpio9.into_floating_input();

        let pulse = PulseControl::new(
            p.RMT,
            &mut system.peripheral_clock_control,
            ClockSource::APB,
            0,
            0,
            0,
        )
        .map_err(|_| IcarusError::HardwareInitError)?;

        let stat = <smartLedAdapter!(1)>::new(pulse.channel0, io.pins.gpio21);

        let battery_sense = io.pins.gpio3.into_push_pull_output();

        let delay = Delay::new(&clocks);

        Ok(Icarus {
            drv1_en,
            drv2_en,
            rtrctl1,
            rtrctl2,
            rtrctl3,
            rtrctl4,
            user_btn,
            stat,
            battery_sense,
            delay,
        })
    }

    pub fn take() -> Result<Icarus, IcarusError> {
        if let Some(p) = Peripherals::take() {
            Icarus::init(p)
        } else {
            Err(IcarusError::HardwareInitError)
        }
    }
}
