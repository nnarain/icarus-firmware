// From esp-idf-hal examples
// Use RMT pulse controller to write to neopixel

use esp_idf_hal::{
    // delay::Ets,
    gpio::OutputPin,
    rmt::{
        config::TransmitConfig,
        FixedLengthSignal, PinState, Pulse, Transmit, HwChannel
    }
};

// use embedded_hal::delay::blocking::DelayUs;

use esp_idf_sys::EspError;

use core::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum StatColor {
    Red, Green, Blue, Black
}

impl From<StatColor> for u32 {
    fn from(c: StatColor) -> Self {
        match c {
            StatColor::Red   => 0x99_00_00u32,
            StatColor::Green => 0x00_99_00u32,
            StatColor::Blue  => 0x00_00_99u32,
            StatColor::Black => 0x00_00_00u32,
        }
    }
}

pub struct StatLed<P: OutputPin, C: HwChannel> {
    tx: Transmit<P, C>,
}

impl<P: OutputPin, C: HwChannel> StatLed<P, C> {
    pub fn new(led: P, chnl: C) -> Result<Self, EspError> {
        let config = TransmitConfig::new().clock_divider(1);
        let tx = Transmit::new(led, chnl, &config)?;

        Ok(StatLed {tx})
    }

    pub fn update(&mut self, c: StatColor) -> Result<(), EspError> {
        let rgb: u32 = c.into();

        let ticks_hz = self.tx.counter_clock()?;
        let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(350))?;
        let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(800))?;
        let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(700))?;
        let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(600))?;

        let mut signal = FixedLengthSignal::<24>::new();

        for i in 0..24 {
            let bit = 2_u32.pow(i) & rgb != 0;
            let (high_pulse, low_pulse) = if bit { (t1h, t1l) } else { (t0h, t0l) };
            signal.set(i as usize, &(high_pulse, low_pulse))?;
        }

        self.tx.start_blocking(&signal)?;

        // Ets.delay_ms(ms)

        Ok(())
    }
}

fn ns(nanos: u64) -> Duration {
    Duration::from_nanos(nanos)
}
