//
// sensors.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 01 2021
//

pub mod imu {
    use mpu6050::Mpu6050;
    use crate::types::I2cBus;

    // Re-exports
    pub type Imu<'a> = Mpu6050<I2cBus<'a>>;

    pub fn init(i2c: I2cBus) -> Imu {
        Mpu6050::new_with_addr(i2c, 0x69)
    }
}
