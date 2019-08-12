// More complicated global-variable initialization, including referring to global variables in other modules

#include "globals_initialization.h"

// forward declarations (these are defined in globals_initialization_1.c)
extern const int a, b, c;
extern const struct SomeStruct ss0;
extern const struct StructWithPointers swp0, swp1;
extern const struct StructWithPointers crossMod0;

// integer constants used by globals_initialization_1.c
const int x = 511;

// struct constants used by globals_initialization_2.c
const struct SomeStruct ss2 = { x * 3, x + 4, 1 };

// referring to constants in another module
const struct StructWithPointers swp2 = { x, &c, &ss2, &swp0 };
const struct StructWithPointers swp3 = { x - 2, &swp0.field1, &ss2, &swp1 };

// a circular data structure, with links across modules
const struct StructWithPointers crossMod1 = { 2, &crossMod0.field1, &ss0, &crossMod0};
