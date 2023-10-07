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

private:
  int16_t pitch_{0};
  int16_t roll_{0};
  int16_t yaw_{0};
};
