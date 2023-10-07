//
// StatLed.hpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 04 2023
//

#pragma once

#include <Adafruit_NeoPixel.h>

class StatLed
{
public:
    StatLed(uint8_t pin);
    ~StatLed() = default;

    bool begin();

    void showConnected();
    void showDisconnected();

private:
    void setColor(uint8_t r, uint8_t g, uint8_t b);

    Adafruit_NeoPixel stat_;
};
