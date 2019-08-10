volatile int global1 = 3;
volatile int global2 = 5;
volatile int global3;

__attribute__((noinline)) int read_global() {
  return global1;
}

__attribute__((noinline)) int modify_global(int x) {
  global3 = x;
  return global3;
}

__attribute__((noinline)) int modify_global_with_call(int x) {
  modify_global(x);
  return global3;
}

int dont_confuse_globals(int x) {
  global1 = 100;
  global2 = 95;
  global3 = x;
  global1 = global2 - 200;
  return global3;
}
