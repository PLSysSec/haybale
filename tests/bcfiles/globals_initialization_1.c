// More complicated global-variable initialization, including referring to global variables in other modules

#include "globals_initialization.h"

// forward declarations (these are defined in globals_initialization_2.c)
extern const int x;
extern const struct SomeStruct ss2;
extern const struct StructWithPointers crossMod1;

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

// a circular data structure, with links across modules
const struct StructWithPointers crossMod0 = { 2, &crossMod1.field1, &ss1, &crossMod1};

// a struct with pointers to functions in the same module
const struct StructWithFunctionPointer swfp1 = { 21, &bar, (void*) &foo };

int foo() {
  return a  // 2
       + b  // 2
       + c  // 6
       + ss0.field1  // 0
       + ss1.field2  // 6
       + ss2.field3  // 1
       + *(swp0.intptr)  // 511
       + swp1.ssptr->field2  // 515
       + swp0.swpptr->swpptr->field1  // 2
       + *(crossMod0.swpptr->swpptr->intptr)  // 2
       + swfp1.funcptr(2, 3);  // 5
}

int bar(int x, int y) {
  return x + y;
}
