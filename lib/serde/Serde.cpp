//
// Serde.cpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 15 2023
//

#include <Serde.hpp>

namespace serde
{
    union FloatToInt
    {
        float f;
        uint32_t i;
    };
    

    uint32_t serialize(uint8_t* buf, uint8_t value)
    {
        buf[0] = value;
        return 1;
    }

    uint32_t serialize(uint8_t* buf, uint16_t value)
    {
        buf[0] = value >> 8;
        buf[1] = value & 0x00FF;
        return 2;
    }

    uint32_t serialize(uint8_t* buf, uint32_t value)
    {
        buf[0] = value >> 24;
        buf[1] = value >> 16;
        buf[2] = value >> 8;
        buf[3] = value & 0x000000FF;
        return 4;
    }


    uint32_t serialize(uint8_t* buf, float value)
    {
        FloatToInt f2i;
        f2i.f = value;

        return serialize(buf, f2i.i);
    }

} // namespace serde

