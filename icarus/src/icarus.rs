//
// board.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 29 2021
//

use crate::{
    hal::{
        self,
        prelude::*,
        gpio::{Pin, Output, U, Gpioa, PushPull}
    },
    IcarusError
};



/// Pinout for icarus controller
pub struct Icarus {
    pub stat1: Pin<Gpioa, U<4>, Output<PushPull>>, // Status LED 1
    pub stat2: Pin<Gpioa, U<5>, Output<PushPull>>, // Status LED 2
}

impl Icarus {
    pub fn new() -> Result<Icarus, IcarusError> {
        let dp = hal::pac::Peripherals::take().unwrap();

        let mut rcc = dp.RCC.constrain();
        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

        // Status LEDs
        let stat1 = gpioa.pa4.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        let stat2 = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

        Ok(
            Icarus {
                stat1,
                stat2,
            }
        )
    }
}

