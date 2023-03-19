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

  // const auto attitude = sensors.getAttitude();
  // Serial.printf("(%0.2f, %0.2f, %0.2f)\n", attitude.pitch, attitude.roll, attitude.yaw);

  // server.updateAttitude(attitude.pitch, attitude.roll, attitude.yaw);

  // const auto& throttle = server.getThrottle();
  // Serial.printf("(%d, %d, %d, %d)\n", throttle.pitch, throttle.roll, throttle.yaw, throttle.vertical);

  // rtrctl.setThrottle(10, 0, 0, 0);
  // delay(5000);
  // rtrctl.setThrottle(0, 0, 0, 0);
  // delay(5000);

  // rtrctl.setThrottle(0, 10, 0, 0);
  // delay(5000);
  // rtrctl.setThrottle(0, 0, 0, 0);
  // delay(5000);

  // rtrctl.setThrottle(0, 0, 10, 0);
  // delay(5000);
  // rtrctl.setThrottle(0, 0, 0, 0);
  // delay(5000);

  // rtrctl.setThrottle(0, 0, 0, 10);
  // delay(5000);
  // rtrctl.setThrottle(0, 0, 0, 0);
  // delay(5000);

  const auto connected = server.isConnected();
  Serial.printf("connected: %d\n", (int)connected);

  delay(1000);
}
