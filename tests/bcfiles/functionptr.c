int foo(int x, int y) {
  return x * (y + 3);
}

__attribute__((noinline)) int calls_fptr(int (*volatile fptr)(int, int), int z) {
  return fptr(2, 3) + z;
}

int fptr_driver() {
  int (*volatile fptr)(int, int) = &foo;
  return calls_fptr(foo, 10);
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
  s.fptr = &foo;
  s.anInt = 3;
  return calls_through_struct(&s);
}
