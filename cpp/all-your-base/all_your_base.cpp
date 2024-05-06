#include <math.h>
#include <stdexcept>
#include <vector>

#include "all_your_base.h"

namespace all_your_base
{
    int toDecimal(unsigned int from, std::vector<unsigned int> digits)
    {
        int decimal = 0;
        for (auto &digit : digits)
        {
            if (digit >= from)
            {
                throw std::invalid_argument("Invalid digit");
            }
            decimal = decimal * from + digit;
        }
        return decimal;
    }

    std::vector<unsigned int> fromDecimal(unsigned int to, int decimal)
    {
        std::vector<unsigned int> digits;
        for (auto rem = decimal; rem > 0; rem /= to)
        {
            digits.insert(digits.begin(), rem % to);
        }
        return digits;
    }

    std::vector<unsigned int> convert(unsigned int from, std::vector<unsigned int> digits, unsigned int to)
    {
        if (from < 2 || to < 2)
        {
            throw std::invalid_argument("Invalid base");
        }
        return fromDecimal(to, toDecimal(from, digits));
    }

} // namespace all_your_base
