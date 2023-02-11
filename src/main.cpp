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

RotorController rtrctl;
Sensors sensors;


void setup() {
  Serial.begin(115200);

  if (!sensors.begin(ICARUS_I2C_SCL, ICARUS_I2C_SDA))
  {
    Serial.println("Failed to initialize sensors!");
    while(1){}
  }
  else
  {
    Serial.println("Sensor setup complete!");
  }
}

void loop() {
  sensors.update();

  const auto attitude = sensors.getAttitude();
  Serial.printf("(%0.2f, %0.2f, %0.2f)\n", attitude.pitch, attitude.roll, attitude.yaw);

  delay(100);
}
