//
// icarus.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 29 2021
//

use crate::{
    hal::{
        self,
        prelude::*,
        pac,
        gpio::{self, Output, PushPull, Alternate},
        serial::Serial,
    },
    IcarusError
};

type PinTx1 = gpio::PA9<Alternate<PushPull, 7>>;
type PinRx1 = gpio::PA10<Alternate<PushPull, 7>>;
type PinTx2 = gpio::PB3<Alternate<PushPull, 7>>;
type PinRx2 = gpio::PB4<Alternate<PushPull, 7>>;

/// Pinout for icarus controller
pub struct Icarus {
    pub stat1: gpio::PA4<Output<PushPull>>,            // Status LED 1
    pub stat2: gpio::PA5<Output<PushPull>>,            // Status LED 2

    pub usart1: Serial<pac::USART1, (PinTx1, PinRx1)>, // Serial Port 1
    pub usart2: Serial<pac::USART2, (PinTx2, PinRx2)>, // Serial Port 2
}

impl Icarus {
    pub fn new() -> Result<Icarus, IcarusError> {
        let dp = hal::pac::Peripherals::take().unwrap();

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
        let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

        // Status LEDs
        let stat1 = gpioa.pa4.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        let stat2 = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

        // Serial ports
        // TODO: Configurable baud rate

        // USART 1
        let tx1 = gpioa.pa9.into_af7_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
        let rx1 = gpioa.pa10.into_af7_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

        let usart1 = Serial::new(dp.USART1, (tx1, rx1), 115200.Bd(), clocks, &mut rcc.apb2);

        // USART 2
        let tx2 = gpiob.pb3.into_af7_push_pull(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
        let rx2 = gpiob.pb4.into_af7_push_pull(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

        let usart2 = Serial::new(dp.USART2, (tx2, rx2), 115200.Bd(), clocks, &mut rcc.apb1);

        Ok(
            Icarus {
                stat1,
                stat2,

                usart1,
                usart2,
            }
        )
    }
}

