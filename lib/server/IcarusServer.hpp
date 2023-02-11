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

private:
    void serializeAttitude();

    NimBLEServer* server_{nullptr};

    // Sensor Service
    NimBLEService* sensor_service_{nullptr};
    NimBLECharacteristic* attitude_characteristic_{nullptr};
    NimBLEDescriptor* attitude_desc_{nullptr};
    AttitudeServiceData attitude_data_;
    uint8_t attitude_service_buffer_[sizeof(AttitudeServiceData)];
};
