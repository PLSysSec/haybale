__attribute__((noinline)) int simple_callee(int x, int y);
__attribute__((noinline)) int simple_caller(int x);

// Identical to simple_caller in call.c.
// The only difference is that this call crosses modules.
__attribute__((noinline)) int cross_module_simple_caller(int x) {
  return simple_callee(x, 3);
}

// Identical to twice_caller in call.c.
// The only difference is that these calls cross modules.
int cross_module_twice_caller(int x) {
  return simple_callee(x, 5) + simple_callee(x, 1);
}

// Nested call on this side
int cross_module_nested_near_caller(int x, int y) {
  return cross_module_simple_caller(x + y);
}

// Nested call on the far side
int cross_module_nested_far_caller(int x, int y) {
  return simple_caller(x + y);
}
