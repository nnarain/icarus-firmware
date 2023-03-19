//
// RotorControl.cpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 04 2023
//
#include <Arduino.h>
#include "RotorController.hpp"

#define RTRCTL_CHNL1 0
#define RTRCTL_CHNL2 1
#define RTRCTL_CHNL3 2
#define RTRCTL_CHNL4 3

#define PWM_FREQ 200
#define PWM_RESOLUTION 8

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

bool RotorController::begin(uint8_t rtr1, uint8_t rtr2, uint8_t rtr3, uint8_t rtr4)
{
    // Setup PWM channels
    ledcSetup(RTRCTL_CHNL1, PWM_FREQ, PWM_RESOLUTION);
    ledcSetup(RTRCTL_CHNL2, PWM_FREQ, PWM_RESOLUTION);
    ledcSetup(RTRCTL_CHNL3, PWM_FREQ, PWM_RESOLUTION);
    ledcSetup(RTRCTL_CHNL4, PWM_FREQ, PWM_RESOLUTION);

    // Attach rotor pins
    ledcAttachPin(rtr1, RTRCTL_CHNL1);
    ledcAttachPin(rtr2, RTRCTL_CHNL2);
    ledcAttachPin(rtr3, RTRCTL_CHNL3);
    ledcAttachPin(rtr4, RTRCTL_CHNL4);

    setThrottle(THROTTLE_MIN, THROTTLE_MIN, THROTTLE_MIN, THROTTLE_MIN);

    return true;
}

void RotorController::setThrottle(uint16_t t1, uint16_t t2, uint16_t t3, uint16_t t4)
{
    const uint16_t t1_actual = constrain(t1 + THROTTLE_MIN, THROTTLE_MIN, THROTTLE_MAX);
    const uint16_t t2_actual = constrain(t2 + THROTTLE_MIN, THROTTLE_MIN, THROTTLE_MAX);
    const uint16_t t3_actual = constrain(t3 + THROTTLE_MIN, THROTTLE_MIN, THROTTLE_MAX);
    const uint16_t t4_actual = constrain(t4 + THROTTLE_MIN, THROTTLE_MIN, THROTTLE_MAX);

    ledcWrite(RTRCTL_CHNL1, t1_actual);
    ledcWrite(RTRCTL_CHNL2, t2_actual);
    ledcWrite(RTRCTL_CHNL3, t3_actual);
    ledcWrite(RTRCTL_CHNL4, t4_actual);
}
