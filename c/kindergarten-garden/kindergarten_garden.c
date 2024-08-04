#include "kindergarten_garden.h"

plants_t plants(const char *diagram, const char *student)
{
    const char *it = diagram;
    while (*(it++) != '\n')
        ;
    plants_t result;
    int index = (*student - 'A');
    result.plants[0] = diagram[2 * index];
    result.plants[1] = diagram[2 * index + 1];
    result.plants[2] = it[2 * index];
    result.plants[3] = it[2 * index + 1];
    return result;
}
