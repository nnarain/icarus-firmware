//
// IcarusServer.cpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 11 2023
//

#include <IcarusServer.hpp>

IcarusServer::IcarusServer()
{
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

    // TODO(nnarain): Setup services

    // Start BLE Advertising
    auto advertising = NimBLEDevice::getAdvertising();
    // TODO(nnarain): Advertise services
    advertising->setScanResponse(true);
    advertising->start();
}
