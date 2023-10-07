//
// ControlInput.hpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Sept 9 2023
//

#pragma once

#include <Bluepad32.h>

class ControlInput
{
public:
  void processGamepad(GamepadPtr gamepad);

  int16_t getPitch() const;
  int16_t getRoll() const;

  int16_t getThrottle() const;

private:
  int16_t pitch_{0};
  int16_t roll_{0};
  int16_t yaw_{0};
  int16_t throttle_{0};
};
