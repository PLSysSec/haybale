// More complicated global-variable initialization, including referring to global variables in other modules

#include "globals_initialization.h"

// forward declarations (these are defined in globals_initialization_2.c)
extern const int x;
extern const struct SomeStruct ss2;

// integer constants
const int a = 2;  // literal constant
const int b = a;  // refers to constant in same file
const int c = b * 3;  // function of constant in same file

// struct constants
const struct SomeStruct ss0 = { 0 };
const struct SomeStruct ss1 = { a, c, b };

// a circular data structure
const struct StructWithPointers swp1;
const struct StructWithPointers swp0 = { b, &x, &ss1, &swp1 };
const struct StructWithPointers swp1 = { c, &swp0.field1, &ss2, &swp0 };

int foo() {
  return a + b + c + ss0.field1 + ss1.field2 + ss2.field3 + *(swp0.intptr) + swp1.ssptr->field2 + swp0.swpptr->swpptr->field1;
      // 2 + 2 + 6 + 0          + 6          + 102        + 6              + 102                + 2
}
