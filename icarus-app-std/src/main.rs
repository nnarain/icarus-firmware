//
// main.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 10 2022
//

use icarus_app_std::stat::{StatLed, StatColor};
use icarus_wire::{self, IcarusState, IcarusCommand, ImuRaw};

use esp_idf_hal::{
    prelude::*,
    peripherals::Peripherals,
    ledc::*,
    i2c,
    delay::FreeRtos,
};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported


use mpu6050::Mpu6050;

use std::{
    sync::{
        Arc,
        mpsc::channel,
    },
    time::Duration,
    thread,
    io::{Write, Read}
};

#[allow(unreachable_code)]
fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // -----------------------------------------------------------------------------------------------------------------
    // Setup Logging
    // -----------------------------------------------------------------------------------------------------------------

    let mut logger = defmt_bbq::init().unwrap();

    defmt::info!("Starting IDLE task");

    // Idle Task
    loop {
        defmt::info!("Hello World!");

        // Write log data to serial port
        match logger.read() {
            Ok(grant) => {
                let buf = grant.buf();
                std::io::stdout().write_all(buf)?;
                std::io::stdout().flush();

                let len = grant.len();
                grant.release(len);
            },
            Err(e) => println!("{:?}", e),
        }

        thread::sleep(Duration::from_millis(1000));
    }

    Ok(())
}
