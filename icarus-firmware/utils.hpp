//
// utils.hpp
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Mar 29 2023
//

namespace utils
{
    inline double mapf(double x, double in_min, double in_max, double out_min, double out_max)
    {
        return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
    }
}
