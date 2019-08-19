#include <stdbool.h>

__attribute__((noinline)) int foo(int x, int y) {
  return x * (y + 3);
}

__attribute__((noinline)) int bar(int x, int y) {
  return x - y;
}

typedef int (*footype)(int, int);

__attribute__((noinline)) int calls_fptr(volatile footype fptr, int z) {
  return fptr(2, 3) + z;
}

__attribute__((noinline)) footype get_function_ptr(bool b) {
  if (b) {
    return &foo;
  } else {
    return &bar;
  }
}

int fptr_driver() {
  int (*volatile fptr)(int, int) = get_function_ptr(true);
  return calls_fptr(fptr, 10);
}

struct StructWithFuncPtr {
  int anInt;
  int (*fptr)(int, int);
};

__attribute((noinline)) int calls_through_struct(volatile struct StructWithFuncPtr *s) {
  return s->fptr(s->anInt, 2);
}

int struct_driver() {
  volatile struct StructWithFuncPtr s = { 0 };
  s.fptr = get_function_ptr(true);
  s.anInt = 3;
  return calls_through_struct(&s);
}
