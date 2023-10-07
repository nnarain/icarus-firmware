//
// Serde.hpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Feb 15 2023
//

#include <cstdint>

namespace serde
{
    uint32_t serialize(uint8_t* buf, uint8_t value);
    uint32_t serialize(uint8_t* buf, uint16_t value);
    uint32_t serialize(uint8_t* buf, uint32_t value);

    uint32_t serialize(uint8_t* buf, float value);

    uint32_t deserialize(const uint8_t* buf, uint8_t& value);
    uint32_t deserialize(const uint8_t* buf, uint16_t& value);
    uint32_t deserialize(const uint8_t* buf, int16_t& value);
    uint32_t deserialize(const uint8_t* buf, uint32_t& value);

    uint32_t deserialize(const uint8_t* buf, float& value);
}
