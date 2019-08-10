// Declarations of functions in call.c
__attribute__((noinline)) int simple_callee(int x, int y);
__attribute__((noinline)) int simple_caller(int x);

// Declarations of functions and globals in globals.c
extern volatile int global1, global2, global3;
__attribute__((noinline)) int read_global();
__attribute__((noinline)) int modify_global(int x);
__attribute__((noinline)) int modify_global_with_call(int x);

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

int cross_module_read_global() {
  return global1;
}

int cross_module_read_global_via_call() {
  return read_global();
}

int cross_module_modify_global(int x) {
  global3 = x;
  return global3;
}

int cross_module_modify_global_via_call(int x) {
  modify_global(x);
  return global3;
}
