//
// RotorControl.hpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 03 2023
//
#pragma once

#include <cstdint>

/**
 * @brief Throttle control for rotors
 *
 */
class RotorController
{
public:
    RotorController() = default;
    ~RotorController() = default;

    bool begin(uint8_t rtr1, uint8_t rtr2, uint8_t rtr3, uint8_t rtr4);

    void setThrottle(uint16_t t1, uint16_t t2, uint16_t t3, uint16_t t4);

private:
};
