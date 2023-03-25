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
#include <IcarusServer.hpp>

RotorController rtrctl;
Sensors sensors;
IcarusServer server;

uint16_t throttle = 0;
uint32_t last_command_time_ms = 0;


void setup() {
  Serial.begin(115200);

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

  server.begin();
  Serial.println("Server setup complete");
}

void loop() {
  sensors.update();
  const auto attitude = sensors.getAttitude();
  server.updateAttitude(attitude.pitch, attitude.roll, attitude.yaw);

  if (server.isConnected())
  {
    // Gradually increase throttle
    const auto now = millis();
    if (now - last_command_time_ms >= 1000)
    {
      rtrctl.setThrottle(throttle, throttle, throttle, throttle);

      throttle = (throttle + 1) % THROTTLE_MAX;
      last_command_time_ms = now;
    }
  }
  else
  {
    rtrctl.setThrottle(0, 0, 0, 0);
  }
}
