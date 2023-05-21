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

//RotorController rtrctl;

#define RTR_PIN 10
#define LED_PIN 7

#define RTRCTL_CHNL1 0
#define PWM_FREQ 200
#define PWM_RESOLUTION 8

void setup() {
  Serial.begin(115200);

  // LED
  pinMode(LED_PIN, OUTPUT);

  // PWM setup
  ledcSetup(RTRCTL_CHNL1, PWM_FREQ, PWM_RESOLUTION);
  ledcAttachPin(RTR_PIN, RTRCTL_CHNL1);

  // Arm
  ledcWrite(RTRCTL_CHNL1, THROTTLE_MIN);
  delay(10000);
  digitalWrite(LED_PIN, HIGH);

  // Command
  ledcWrite(RTRCTL_CHNL1, THROTTLE_MIN + 10);
}

void loop() {
  // sensors.update();
  // const auto attitude = sensors.getAttitude();
  // server.updateAttitude(attitude.pitch, attitude.roll, attitude.yaw);

  // if (server.isConnected())
  // {
  //   const auto throttle = server.getThrottle();
  //   rtrctl.setCommand(throttle.pitch, throttle.roll, throttle.yaw, throttle.vertical);
  // }
  // else
  // {
  //   rtrctl.setCommand(0, 0, 0, 0);
  // }
}
