//
// Sensors.hpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 03 2023
//

#pragma once

#include <cstdint>

#include <MPU6050.h>

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

    attitude_t getAttitude();
private:
    MPU6050 mpu;
};
