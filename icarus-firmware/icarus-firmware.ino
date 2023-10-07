//
// Firmware for Icarus Flight Controller
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 03 2023
//

#include <Arduino.h>

#include <Bluepad32.h>

#include "pins.hpp"

#include "ControlInput.hpp"
#include "RotorController.hpp"
#include "Sensors.hpp"
#include "StatLed.hpp"

ControlInput input;
RotorController rtrctl;
Sensors sensors;
StatLed led{ICARUS_STAT_LED};

uint16_t throttle = 0;
uint32_t last_command_time_ms = 0;
uint32_t last_connected_time_ms = 0;

uint32_t armed_time_ms = 0;

GamepadPtr connected_gamepad = nullptr;


void setup() {
  // Setup serial logging
  Serial.begin(115200);
  Serial.setPins(ICARUS_UART_RX, ICARUS_UART_TX);

  // Initialize rotor control
  rtrctl.begin(ICARUS_IO1, ICARUS_IO2, ICARUS_IO3, ICARUS_IO4);

  // Initialize sensors
  if (!sensors.begin(ICARUS_I2C_SCL, ICARUS_I2C_SDA))
  {
    Serial.println("Failed to initialize sensors!");
    while(1){}
  }
  else
  {
    Serial.println("Sensor setup complete!");
  }

  // Initialize stat led
  led.begin();

  // Initialize gamepad support
  BP32.setup(&onConnectedGamepad, &onDisconnectedGamepad);

  // Add this for now
  BP32.forgetBluetoothKeys();
}

void loop() {
  sensors.update();

  // Get the estimated state
  const auto attitude = sensors.getAttitude();

  const auto pitch = attitude.pitch;
  const auto roll = attitude.roll;
  const auto yaw = attitude.yaw;

  // Update the controller with the estimated state
  rtrctl.update(pitch, roll, yaw);

  // const auto now = millis();
  // if (now > 10000)
  // {
  //   static int16_t throttle = 0;

  //   if (now >= last_command_time_ms + 100 && throttle < 50)
  //   {
  //     rtrctl.setCommand(0, 0, 0, throttle);
  //     throttle++;
  //     last_command_time_ms = now;
  //   }
  // }

  // Update input from gamepad
  BP32.update();
  processGamepad(connected_gamepad);

  // Needed for BT task to get time
  delay(75);
}

void processGamepad(GamepadPtr gamepad)
{
  if (gamepad && gamepad->isConnected())
  {
    input.processGamepad(gamepad);
    rtrctl.setCommand(input.getPitch(), input.getRoll(), 0, input.getThrottle());
  }
  else
  {
    // rtrctl.setCommand(0, 0, 0, 0);
    rtrctl.disarm();
  }
}

void onConnectedGamepad(GamepadPtr gp)
{
  if (!connected_gamepad && gp->isGamepad())
  {
    connected_gamepad = gp;
    led.showConnected();
    Serial.println("Gamepad Connected");
  }
}

void onDisconnectedGamepad(GamepadPtr gp)
{
  if (connected_gamepad == gp)
  {
    connected_gamepad = nullptr;
    led.showDisconnected();
  }
}
