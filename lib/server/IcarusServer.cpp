//
// IcarusServer.cpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 11 2023
//
#include <Arduino.h>
#include <IcarusServer.hpp>

#include <Serde.hpp>

#define ICARUS_SENSOR_SERVICE_UUID                         "243a63eb-17cc-444c-b3bd-1302b399a7a0"
#define ICARUS_SENSOR_SERVICE_CHARACTERISTIC_ATTITUDE_UUID "68af1093-1df9-41ac-98e8-d524a025b4b9"

#define ICARUS_THROTTLE_SERVICE_UUID                         "701748c6-7c50-40b6-b30e-98da2646070d"
#define ICARUS_THROTTLE_SERVICE_CHARACTERISTIC_THROTTLE_UUID "c346b87e-9a11-4a56-9a53-e421c8ade193"

/**
 * @brief Connection status updater
 *
 */
class ConnectionStateUpdater : public NimBLEServerCallbacks
{
public:
    ConnectionStateUpdater(bool& connected) : connected_{connected} {}

    void onConnect(NimBLEServer* server) override
    {
        connected_ = true;
    }

    void onDisconnect(NimBLEServer* server) override
    {
        connected_ = false;
    }

private:
    bool& connected_;
};

/**
 * @brief Construct a new Icarus Server:: Icarus Server object
 *
 */
class ThrottleDecoder : public NimBLECharacteristicCallbacks
{
public:
    ThrottleDecoder(Throttle* throttle)
        : throttle_{throttle}
    {
    }

    void onWrite(NimBLECharacteristic* throttle)
    {
        auto buf = throttle->getValue().data();
        buf += serde::deserialize(buf, throttle_->pitch);
        buf += serde::deserialize(buf, throttle_->roll);
        buf += serde::deserialize(buf, throttle_->yaw);
        buf += serde::deserialize(buf, throttle_->vertical);
    }
private:
    Throttle* throttle_{nullptr};
};

IcarusServer::IcarusServer()
{
    memset(&attitude_service_buffer_, 0, sizeof(attitude_service_buffer_));
}

IcarusServer::~IcarusServer()
{
}

void IcarusServer::begin()
{
    // Set the device name
    NimBLEDevice::init("icarus");

    // Setup the server
    server_ = NimBLEDevice::createServer();
    server_->setCallbacks(new ConnectionStateUpdater{connected_});

    // Setup services

    // Sensors
    sensor_service_ = server_->createService(ICARUS_SENSOR_SERVICE_UUID);
    attitude_characteristic_ = sensor_service_->createCharacteristic(
                                            ICARUS_SENSOR_SERVICE_CHARACTERISTIC_ATTITUDE_UUID,
                                            NIMBLE_PROPERTY::READ | NIMBLE_PROPERTY::NOTIFY
                                        );

    updateAttitude(0, 0, 0);

    // Throttle
    throttle_service_ = server_->createService(ICARUS_THROTTLE_SERVICE_UUID);
    throttle_characteristic_ = throttle_service_->createCharacteristic(
                                            ICARUS_THROTTLE_SERVICE_CHARACTERISTIC_THROTTLE_UUID,
                                            NIMBLE_PROPERTY::READ | NIMBLE_PROPERTY::WRITE
                                        );
    throttle_characteristic_->setValue(throttle_data_, sizeof(Throttle));
    throttle_characteristic_->setCallbacks(new ThrottleDecoder{&throttle_});

    // Starting Services
    sensor_service_->start();
    throttle_service_->start();

    // Start BLE Advertising
    auto advertising = NimBLEDevice::getAdvertising();

    advertising->addServiceUUID(sensor_service_->getUUID());
    advertising->addServiceUUID(throttle_service_->getUUID());

    advertising->setScanResponse(true);
    advertising->start();
}

void IcarusServer::updateAttitude(float pitch, float roll, float yaw)
{
    attitude_data_.pitch = pitch;
    attitude_data_.roll = roll;
    attitude_data_.yaw = yaw;

    serializeAttitude();
}

void IcarusServer::serializeAttitude()
{
    uint8_t* buf = attitude_service_buffer_;

    buf += serde::serialize(buf, attitude_data_.pitch);
    buf += serde::serialize(buf, attitude_data_.roll);
    buf += serde::serialize(buf, attitude_data_.yaw);

    attitude_characteristic_->setValue(attitude_service_buffer_, sizeof(attitude_service_buffer_));
    attitude_characteristic_->notify();
}

bool IcarusServer::isConnected() const
{
    return connected_;
}

const Throttle& IcarusServer::getThrottle() const
{
    return throttle_;
}
