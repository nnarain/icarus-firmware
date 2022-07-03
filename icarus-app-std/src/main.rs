//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 03 2022
//

use esp_idf_hal::peripherals::Peripherals;
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let p = Peripherals::take()?;
    let mut led = p.pins.gpio10.into_output();

    loop {}
}
