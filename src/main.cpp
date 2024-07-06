//
// Firmware for Icarus Flight Controller
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 03 2023
//

#include <Arduino.h>

#include <pins.hpp>

#include <RotorController.hpp>
#include <Sensors.hpp>
#include <StatLed.hpp>

RotorController rtrctl;
Sensors sensors;
StatLed led{ICARUS_STAT_LED};

uint16_t throttle = 0;
uint32_t last_command_time_ms = 0;
uint32_t last_connected_time_ms = 0;


void setup() {
  Serial.begin(115200);
  Serial.setPins(ICARUS_UART_RX, ICARUS_UART_TX);

  rtrctl.begin(ICARUS_IO1, ICARUS_IO2, ICARUS_IO3, ICARUS_IO4);

  if (!sensors.begin(ICARUS_I2C_SCL, ICARUS_I2C_SDA))
  {
    Serial.println("Failed to initialize sensors!");
    while(1){}
  }
  else
  {
    Serial.println("Sensor setup complete!");
  }

  led.begin();
}

void loop() {
  sensors.update();
  const auto attitude = sensors.getAttitude();

  // if (server.isConnected())
  // {
  //   const auto throttle = server.getThrottle();
  //   rtrctl.setCommand(throttle.pitch, throttle.roll, throttle.yaw, throttle.vertical);

  //   led.showConnected();
  // }
  // else
  // {
  //   rtrctl.disarm();

  //   led.showDisconnected();
  // }

  const auto now = millis();

  if (now > 10000)
  {
    led.showConnected();

    if (now >= (last_command_time_ms + 100) && throttle < 50)
    {
      rtrctl.setCommand(0, 0, 0, throttle);
      throttle += 1;

      // if (throttle > 100)
      // {
      //   throttle = 0;
      // }

      last_command_time_ms = now;
    }
  }

  // Get the estimated state
  const auto pitch = attitude.pitch;
  const auto roll = attitude.roll;
  const auto yaw = attitude.yaw;

  // Update the controller with the estimated state
  rtrctl.update(pitch, roll, yaw);
}
