//
// Sensors.cpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 03 2023
//

#include <Arduino.h>
#include "Sensors.hpp"

#include <Wire.h>

// IMU Address
#define MPU_ADDRESS 0x68
// Magnetometer Address
#define BMM_ADDRESS 0x10
// Barometer Address
#define BMP_ADDRESS 0x77

bool Sensors::begin(uint8_t scl, uint8_t sda)
{
    const auto wire_ok = Wire.setPins(sda, scl) && Wire.begin();
    // Wire.setClock()

    if (!wire_ok)
    {
        Serial.println("Failed to initialize Wire protocol");
    }

    mpu.setDeviceAddress(MPU_ADDRESS);

    const auto mpu_ok = mpu.begin();

    if (!mpu_ok)
    {
        Serial.println("Failed to initialize MPU");
    }
    else
    {
        mpu.calibrate();
    }

    return wire_ok && mpu_ok;
}

void Sensors::update()
{
    mpu.update();
}

attitude_t Sensors::getAttitude()
{
    return mpu.getAttitude();
}