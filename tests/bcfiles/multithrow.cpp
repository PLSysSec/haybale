#include <cstdint>

int multi_throw_debug(volatile int a) {
    switch (a % 5) {
        case 1: return 1;
        case 2: return 2;
        case 3: throw (int32_t)3;
        case 4: throw (int32_t)3; // testing distinctness
        default: throw (int32_t)4;
    }
}
