//
// Sensors.hpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 03 2023
//

#pragma once

#include <Adafruit_Sensor_Calibration.h>
#include <Adafruit_MPU6050.h>
#include <Adafruit_AHRS.h>

#include <cstdint>

struct Attitude
{
  float pitch{0};
  float roll{0};
  float yaw{0};
};

/**
 * @brief Icarus sensors
 *
 */
class Sensors
{
public:
    Sensors() = default;
    ~Sensors() = default;

    bool begin(uint8_t scl, uint8_t sda);
    void update();

    Attitude getAttitude();
private:
    Adafruit_MPU6050 mpu_;

    Adafruit_Sensor* accel_{nullptr};
    Adafruit_Sensor* gyro_{nullptr};

    Adafruit_Madgwick filter_;
    uint32_t last_filter_update_;

    Attitude attitude_;
};
