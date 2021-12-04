//
// icarus.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Nov 29 2021
//

use core::convert::TryInto;

use crate::{
    hal::{
        self,
        prelude::*,
        pac,
        gpio::{self, Output, PushPull, Alternate, OpenDrain, Input},
        serial::Serial,
        i2c,
        delay::Delay,
        pwm,
    },
    IcarusError
};

use shared_bus::{BusManagerSimple, I2cProxy, NullMutex};

type PinTx1 = gpio::PA9<Alternate<PushPull, 7>>;
type PinRx1 = gpio::PA10<Alternate<PushPull, 7>>;
type PinTx2 = gpio::PB3<Alternate<PushPull, 7>>;
type PinRx2 = gpio::PB4<Alternate<PushPull, 7>>;
type PinScl = gpio::PB6<Alternate<OpenDrain, 4>>;
type PinSda = gpio::PB7<Alternate<OpenDrain, 4>>;

pub type I2c = i2c::I2c<pac::I2C1, (PinScl, PinSda)>;
pub type I2cBus<'a> = I2cProxy<'a, NullMutex<I2c>>;

/// Pinout for icarus controller
pub struct Icarus {
    pub stat1: gpio::PA4<Output<PushPull>>,            // Status LED 1
    pub stat2: gpio::PA5<Output<PushPull>>,            // Status LED 2

    pub usart1: Serial<pac::USART1, (PinTx1, PinRx1)>, // Serial Port 1
    pub usart2: Serial<pac::USART2, (PinTx2, PinRx2)>, // Serial Port 2

    pub i2c: BusManagerSimple<I2c>,                    // I2C

    pub pwm1: pwm::PwmChannel<pwm::Tim2Ch1, pwm::WithPins>,
    pub pwm2: pwm::PwmChannel<pwm::Tim2Ch2, pwm::WithPins>,
    pub pwm3: pwm::PwmChannel<pwm::Tim2Ch3, pwm::WithPins>,
    pub pwm4: pwm::PwmChannel<pwm::Tim2Ch4, pwm::WithPins>,
    pub pwm6: pwm::PwmChannel<pwm::Tim17Ch1, pwm::WithPins>,

    pub d1: gpio::PB8<Input>,
    pub d2: gpio::PB9<Input>,
    pub d3: gpio::PB10<Input>,
    pub d4: gpio::PB11<Input>,
    pub d5: gpio::PB12<Input>,

    pub sck: gpio::PB13<Input>,
    pub miso: gpio::PB14<Input>,
    pub mosi: gpio::PB15<Input>,

    pub delay: Delay,
}

impl Icarus {
    pub fn new() -> Result<Icarus, IcarusError> {
        let cp = hal::pac::CorePeripherals::take().unwrap();
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

        // I2C
        let scl = gpiob.pb6.into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);
        let sda = gpiob.pb7.into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

        let i2c = i2c::I2c::new(dp.I2C1, (scl, sda), 400.kHz().try_into().unwrap(), clocks, &mut rcc.apb1);
        let i2c = BusManagerSimple::new(i2c);

        // PWM
        let pwm_pin1 = gpioa.pa0.into_af1_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
        let pwm_pin2 = gpioa.pa1.into_af1_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
        let pwm_pin3 = gpioa.pa2.into_af1_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
        let pwm_pin4 = gpioa.pa3.into_af1_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
        let _pwm_pin5 = gpioa.pa6.into_af1_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
        let pwm_pin6 = gpioa.pa7.into_af1_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

        // Setup timer channels for PWM
        // TODO(nnarain): Define what the resolution should be
        // TODO(nnarain): Configurable frequency
        let tim2 = pwm::tim2(dp.TIM2, 1280, 50.Hz(), &clocks);
        let _tim16 = pwm::tim16(dp.TIM16, 1280, 50.Hz(), &clocks);
        let tim17 = pwm::tim17(dp.TIM17, 1280, 50.Hz(), &clocks);

        let pwm1 = tim2.0.output_to_pa0(pwm_pin1);
        let pwm2 = tim2.1.output_to_pa1(pwm_pin2);
        let pwm3 = tim2.2.output_to_pa2(pwm_pin3);
        let pwm4 = tim2.3.output_to_pa3(pwm_pin4);
        // let pwm5 = tim16.output_to_pa6(p)
        let pwm6 = tim17.output_to_pa7(pwm_pin6);

        // GPIO + SPI pins
        let d1 = gpiob.pb8;
        let d2 = gpiob.pb9;
        let d3 = gpiob.pb10;
        let d4 = gpiob.pb11;
        let d5 = gpiob.pb12;

        let sck = gpiob.pb13;
        let miso = gpiob.pb14;
        let mosi = gpiob.pb15;

        // Delay
        let delay = Delay::new(cp.SYST, clocks);

        Ok(
            Icarus {
                stat1,
                stat2,

                usart1,
                usart2,

                i2c,

                pwm1,
                pwm2,
                pwm3,
                pwm4,
                pwm6,

                d1,
                d2,
                d3,
                d4,
                d5,

                sck,
                miso,
                mosi,

                delay,
            }
        )
    }
}

