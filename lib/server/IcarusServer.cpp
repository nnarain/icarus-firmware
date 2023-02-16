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

    // Setup services
    sensor_service_ = server_->createService(ICARUS_SENSOR_SERVICE_UUID);
    attitude_characteristic_ = sensor_service_->createCharacteristic(
                                            ICARUS_SENSOR_SERVICE_CHARACTERISTIC_ATTITUDE_UUID,
                                            NIMBLE_PROPERTY::READ | NIMBLE_PROPERTY::NOTIFY
                                        );

    updateAttitude(0, 0, 0);

    // Starting Services
    sensor_service_->start();

    // Start BLE Advertising
    auto advertising = NimBLEDevice::getAdvertising();

    advertising->addServiceUUID(sensor_service_->getUUID());

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
