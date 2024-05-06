#if !defined(ALL_YOUR_BASE_H)
#define ALL_YOUR_BASE_H
#include <vector>

namespace all_your_base
{
    int toDecimal(unsigned int from, std::vector<unsigned int> digits);
    std::vector<unsigned int> fromDecimal(unsigned int to, int decimal);
    std::vector<unsigned int> convert(unsigned int from, std::vector<unsigned int> digits, unsigned int to);
} // namespace all_your_base

#endif // ALL_YOUR_BASE_H
