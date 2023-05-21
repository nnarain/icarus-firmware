//
// RotorControl.hpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 03 2023
//
#pragma once

#include <PID_v1.h>

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
    static constexpr double KP = 2.0;
    static constexpr double KI = 5.0;
    static constexpr double KD = 1.0;

    RotorController();
    ~RotorController() = default;

    bool begin(uint8_t rtr1, uint8_t rtr2, uint8_t rtr3, uint8_t rtr4);

    void update(double pitch, double roll, double yaw);

    void setCommand(int16_t t1, int16_t t2, int16_t t3, int16_t t4);
    // void update();

    void setThrottle(uint16_t t1, uint16_t t2, uint16_t t3, uint16_t t4);

private:
    

    // Pitch PID Controller
    double pitch_input_{0.0};
    double pitch_setpoint_{0.0};
    double pitch_output_{0.0};
    PID pitch_pid_;

    // Roll PID Controller
    double roll_input_{0.0};
    double roll_setpoint_{0.0};
    double roll_output_{0.0};
    PID roll_pid_;

    // Yaw PID Controller
    double yaw_input_{0.0};
    double yaw_setpoint_{0.0};
    double yaw_output_{0.0};
    PID yaw_pid_;

    // Overall Throttle
    double throttle_{0.0};
};
