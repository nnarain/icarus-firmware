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

// Filter update rate
#define FILTER_UPDATE_RATE 100.0
#define FILTER_UPDATE_PERIOD_MS ((1.0 / FILTER_UPDATE_RATE) * 1000)

bool Sensors::begin(uint8_t scl, uint8_t sda)
{
    const auto wire_ok = Wire.setPins(sda, scl) && Wire.begin();
    // Wire.setClock()

    if (!wire_ok)
    {
        Serial.println("Failed to initialize Wire protocol");
        return false;
    }

    const auto mpu_ok = mpu_.begin(MPU_ADDRESS);
    if (!mpu_ok)
    {
      Serial.println("Failed to initialize IMU");
      return false;
    }

    accel_ = mpu_.getAccelerometerSensor();
    gyro_ = mpu_.getGyroSensor();

    filter_.begin(FILTER_UPDATE_RATE);

    return true;
}

void Sensors::update()
{
  const auto now = millis();

  if ((now - last_filter_update_) >= FILTER_UPDATE_PERIOD_MS)
  {
    float ax, ay, az;
    float gx, gy, gz;

    // Read sensor data
    sensors_event_t accel_event;
    sensors_event_t gyro_event;

    accel_->getEvent(&accel_event);
    gyro_->getEvent(&gyro_event);

    ax = accel_event.acceleration.x;
    ay = accel_event.acceleration.y;
    az = accel_event.acceleration.z;

    // Convert units from the gyro
    gx = gyro_event.gyro.x * SENSORS_RADS_TO_DPS;
    gy = gyro_event.gyro.y * SENSORS_RADS_TO_DPS;
    gz = gyro_event.gyro.z * SENSORS_RADS_TO_DPS;

    // Fuse accelerometer and gyro data
    filter_.updateIMU(gx, gy, gz, ax, ay, az);

    // Get the estimated position
    attitude_.pitch = filter_.getPitch();
    attitude_.roll = filter_.getRoll();
    attitude_.yaw = filter_.getYaw();

    last_filter_update_ = now;
  }
}

Attitude Sensors::getAttitude()
{
    return attitude_;
}