//
// ControlInput.cpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Sept 9 2023
//

#include "ControlInput.hpp"

#include "utils.hpp"

void ControlInput::processGamepad(GamepadPtr gamepad)
{
  const auto lx_axis = gamepad->axisX();
  const auto ly_axis = gamepad->axisY();
  const auto rx_axis = gamepad->axisRX();
  const auto ry_axis = gamepad->axisRY();

  pitch_ = (int16_t)utils::mapf((double)ry_axis, -512.0, 512.0, -100.0, 100.0);
  roll_ = (int16_t)utils::mapf((double)rx_axis, -512.0, 512.0, -100.0, 100.0);
  throttle_ = (int16_t)utils::mapf((double)ly_axis, -512.0, 512.0, -100.0, 100.0) * -1;
}

int16_t ControlInput::getPitch() const
{
  return pitch_;
}

int16_t ControlInput::getRoll() const
{
  return roll_;
}

int16_t ControlInput::getThrottle() const
{
  return throttle_;
}
