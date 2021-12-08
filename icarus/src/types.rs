//
// types.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 06 2021
//
use crate::hal::{
    pac,
    gpio::{self, Output, PushPull, OpenDrain},
    i2c,
    serial::Serial,
};

use shared_bus::{I2cProxy, NullMutex};

pub type PinStat1 = gpio::PA4<Output<PushPull>>;
pub type PinStat2 = gpio::PA5<Output<PushPull>>;

pub type PinTx1 = gpio::PA9<gpio::AF7<PushPull>>;
pub type PinRx1 = gpio::PA10<gpio::AF7<PushPull>>;
pub type PinTx2 = gpio::PB3<gpio::AF7<PushPull>>;
pub type PinRx2 = gpio::PB4<gpio::AF7<PushPull>>;
pub type PinScl = gpio::PB6<gpio::AF4<OpenDrain>>;
pub type PinSda = gpio::PB7<gpio::AF4<OpenDrain>>;

pub type I2c = i2c::I2c<pac::I2C1, (PinScl, PinSda)>;
pub type I2cBus<'a> = I2cProxy<'a, NullMutex<I2c>>;

pub type Serial1 = Serial<pac::USART1, (PinTx1, PinRx1)>;
pub type Serial2 = Serial<pac::USART2, (PinTx2, PinRx2)>;
