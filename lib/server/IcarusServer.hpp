//
// IcarusServer.hpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 11 2023
//

#pragma once

#include <NimBLEDevice.h>

struct AttitudeServiceData
{
    AttitudeServiceData() : pitch{0}, roll{0}, yaw{0} {}

    float pitch;
    float roll;
    float yaw;
};

struct Throttle
{
    int16_t pitch{0};
    int16_t roll{0};
    int16_t yaw{0};
    int16_t vertical{0};
};

/**
 * @brief Icarus BLE Server
 *
 */
class IcarusServer
{
public:
    IcarusServer();
    ~IcarusServer();

    void begin();

    void updateAttitude(float pitch, float roll, float yaw);

    bool isConnected() const;
    const Throttle& getThrottle() const;

private:
    void serializeAttitude();

    NimBLEServer* server_{nullptr};
    bool connected_{false};

    // Sensor Service
    NimBLEService* sensor_service_{nullptr};
    NimBLECharacteristic* attitude_characteristic_{nullptr};
    AttitudeServiceData attitude_data_;
    uint8_t attitude_service_buffer_[sizeof(AttitudeServiceData)];

    // Throttle
    NimBLEService* throttle_service_{nullptr};
    NimBLECharacteristic* throttle_characteristic_{nullptr};
    uint8_t throttle_data_[sizeof(Throttle)];
    Throttle throttle_;
};
