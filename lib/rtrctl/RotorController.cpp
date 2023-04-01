//
// RotorControl.cpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 04 2023
//
#include <Arduino.h>
#include "RotorController.hpp"
#include "utils.hpp"

#define RTRCTL_CHNL1 0
#define RTRCTL_CHNL2 1
#define RTRCTL_CHNL3 2
#define RTRCTL_CHNL4 3

#define PWM_FREQ 200
#define PWM_RESOLUTION 8

#define PITCH_MIN -10
#define PITCH_MAX 10
#define ROLL_MIN -10
#define ROLL_MAX 10
#define YAW_MIN -10
#define YAW_MAX 10


RotorController::RotorController()
    : pitch_pid_{&pitch_input_, &pitch_setpoint_, &pitch_output_, KP, KI, KD, DIRECT}
    , roll_pid_{&roll_input_, &roll_setpoint_, &roll_output_, KP, KI, KD, DIRECT}
    , yaw_pid_{&yaw_input_, &yaw_setpoint_, &yaw_output_, KP, KI, KD, DIRECT}
{
}

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

void RotorController::update(double pitch, double roll, double yaw)
{
    pitch_input_ = pitch;
    roll_input_ = roll;
    yaw_input_ = yaw;

    const auto pitch_updated = pitch_pid_.Compute();
    const auto roll_updated = roll_pid_.Compute();
    // const auto yaw_updated = yaw_pid_.Compute();

    const auto output_updated = pitch_updated || roll_updated;

    if (output_updated)
    {
        /*
            Rotor Layout

              ^^
           (4)  (2)
              \/
              /\
           (3)  (1)
        */
        const auto t1 = throttle_ + pitch_output_ + roll_output_ - yaw_output_;
        const auto t2 = throttle_ - pitch_output_ + roll_output_ + yaw_output_;
        const auto t3 = throttle_ + pitch_output_ - roll_output_ + yaw_output_;
        const auto t4 = throttle_ - pitch_output_ - roll_output_ - yaw_output_;

        setThrottle(t1, t2, t3, t4);
    }
}

void RotorController::setCommand(int16_t pitch, int16_t roll, int16_t yaw, int16_t throttle)
{
    // Map input to angles to control direction
    //
    // Max input range: [-100, +100]

    pitch_setpoint_ = utils::mapf((double)pitch, -100.0, 100.0, -10.0, 10.0);
    roll_setpoint_ = utils::mapf((double)roll, -100.0, 100.0, -10.0, 10.0);

    // TODO(nnarain): Yaw maybe angular rate control...
    yaw_setpoint_ = utils::mapf((double)yaw, -100.0, 100.0, -10.0, 10.0);

    throttle_ = (double)throttle;
}

void RotorController::setThrottle(uint16_t t1, uint16_t t2, uint16_t t3, uint16_t t4)
{
    // Constrain throttle
    const uint16_t t1_actual = constrain(t1, THROTTLE_MIN, THROTTLE_MAX);
    const uint16_t t2_actual = constrain(t2, THROTTLE_MIN, THROTTLE_MAX);
    const uint16_t t3_actual = constrain(t3, THROTTLE_MIN, THROTTLE_MAX);
    const uint16_t t4_actual = constrain(t4, THROTTLE_MIN, THROTTLE_MAX);

    ledcWrite(RTRCTL_CHNL1, t1_actual);
    ledcWrite(RTRCTL_CHNL2, t2_actual);
    ledcWrite(RTRCTL_CHNL3, t3_actual);
    ledcWrite(RTRCTL_CHNL4, t4_actual);
}
