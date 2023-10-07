//
// StatLed.cpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 04 2023
//

#include "StatLed.hpp"

StatLed::StatLed(uint8_t pin)
    : stat_{1, pin, NEO_GBR | NEO_KHZ800}
{
}

bool StatLed::begin()
{
    stat_.begin();
    setColor(0xFF, 0xFF, 0x00);
    stat_.show();

    return true;
}


void StatLed::showConnected()
{
    setColor(0x00, 0xFF, 0x00);
}
void StatLed::showDisconnected()
{
    setColor(0xFF, 0x00, 0x00);
}

void StatLed::setColor(uint8_t r, uint8_t g, uint8_t b)
{
    stat_.setPixelColor(0, stat_.Color(r, g, b));
    stat_.show();
}
