__attribute__((noinline)) int simple_callee(int x, int y) {
  return x - y;
}

__attribute__((noinline)) int simple_caller(int x) {
  return simple_callee(x, 3);
}

int conditional_caller(int x, int y) {
  if (y > 5) {
    return simple_callee(x, 3);
  } else {
    return y + 10;
  }
}

int twice_caller(int x) {
  return simple_callee(x, 5) + simple_callee(x, 1);
}

int nested_caller(int x, int y) {
  return simple_caller(x + y);
}

__attribute__((noinline)) int callee_with_loop(int x, int y) {
  volatile int a = 0;
  for (volatile int i = 0; i < x; i++) {
    a += 10;
  }
  return a - (y + 27);
}

int caller_of_loop(int x) {
  return callee_with_loop(x, 3);
}

int caller_with_loop(int x) {
  int a = 0;
  for (volatile int i = 0; i < x; i++) {
    a += simple_callee(a + 3, 1);
  }
  return a - 14;
}

__attribute__((noinline)) int recursive_simple(int x) {
  int y = x * 2;
  if (y > 25) return y;
  return recursive_simple(y) - 38;
}

__attribute__((noinline)) int recursive_more_complicated(int x) {
  int y = x * 2;
  if (y > 25) {
    return recursive_more_complicated(y % 7) + 1;
  } else if (y < -10) {
    return recursive_more_complicated(-y) - 1;
  } else {
    return y - 14;
  }
}

__attribute__((noinline)) int recursive_not_tail(int x) {
  if (x > 7) return x + 10;
  int a = recursive_not_tail(x + 2);
  if (a % 2 == 0) return (a / 6) - 3;
  return a - 8;
}

__attribute__((noinline)) int recursive_and_normal_caller(int x) {
  int y = x * 2;
  if (simple_callee(y, 3) > 25) return y;
  return recursive_and_normal_caller(y) - 38;
}

__attribute__((noinline)) int mutually_recursive_b(int x);
__attribute__((noinline)) int mutually_recursive_a(int x) {
  if (x > 3) return x;
  return mutually_recursive_b(x / 2) - 1;
}

__attribute__((noinline)) int mutually_recursive_b(int x) {
  if (x < 0) return x;
  return mutually_recursive_a(x + 5) - 9;
}
