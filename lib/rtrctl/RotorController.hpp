//
// RotorControl.hpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 03 2023
//
#pragma once

#include <cstdint>

// 200Hz -> 5ms
// 8-bit resolution -> 255 steps
// 5ms / 255 -> 0.0196078431372549 ms per step
//
// Max Throttle -> 2ms pulse width
// 2ms / 0.0196078431372549 = 102
//
// Min Throttle -> 1ms pulse width
// 1ms / 0.0196078431372549 = 51

#define THROTTLE_MIN 51
#define THROTTLE_MAX 102

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
