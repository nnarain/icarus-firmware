//
// IcarusServer.hpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 11 2023
//

#pragma once

#include <NimBLEDevice.h>

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

private:
    NimBLEServer* server_{nullptr};
};
