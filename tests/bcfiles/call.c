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
  if (x < -1000) {
    return -1;
  } else if (y > 25) {
    return y;
  } else {
    return recursive_simple(y) - 44;
  }
}

__attribute__((noinline)) int recursive_double(int x) {
  int y = x * 2;
  if (x < -1000) {
    return -1;
  } else if (y > 1000) {
    return y;
  } else if (y > 25) {
    return recursive_double(y + 7) + 1;
  } else if (y < -10) {
    return recursive_double(-y) - 1;
  } else {
    return y - 23;
  }
}

__attribute__((noinline)) int recursive_not_tail(int x) {
  if (x > 100) return x + 10;
  int a = recursive_not_tail(x + 20);
  if (a % 2 == 0) return a % 3;
  return (a % 5) - 8;
}

__attribute__((noinline)) int recursive_and_normal_caller(int x) {
  if (x < 0) return -1;
  int y = x * 2;
  if (simple_callee(y, 3) > 25) return y;
  return recursive_and_normal_caller(y) - 44;
}

__attribute__((noinline)) int mutually_recursive_b(int x);
__attribute__((noinline)) int mutually_recursive_a(int x) {
  int u = 5;
  if (x > u) return x;
  return mutually_recursive_b(x * 2) - 1;
}

__attribute__((noinline)) int mutually_recursive_b(int x) {
  int j = 2;
  int k = 2;
  if (x < 0) return x;
  return mutually_recursive_a(x - k) - j;
}

// for mutually_recursive_a(x) to return 0,
//   x must be <= u and mutually_recursive_b(x*2) must be 1
//   x must be <= u and x*2 must be >= 0 and mutually_recursive_a(x*2 - k) must be j+1
//   0 <= x <= u and mutually_recursive_a(2x - k) = j+1
//   0 <= x <= u and (2x - k) <= u and mutually_recursive_b((2x - k)*2) = j+2
//   0 <= x <= u and (2x - k) <= u and mutually_recursive_b(4x - 2k) = j+2
//   0 <= x <= u and (2x - k) <= u and (4x - 2k) >= 0 and mutually_recursive_a(4x - 2k - k) = 2j+2
//   0 <= x <= u and (2x - k) <= u and (2x - k) >= 0 and mutually_recursive_a(4x - 3k) = 2j+2
//   0 <= x <= u and 0 <= (2x - k) <= u and mutually_recursive_a(4x - 3k) = 2j+2
//     if the recursion ends here then u < 2j+2 and 4x - 3k = 2j+2
//     0 <= x <= u < 2j+2 = 4x - 3k and 0 <= 2x - k <= u
//     Any satisfying solution must have x < 4x - 3k
//                                       0 < x - k
//                                       k < x
//     Try x = 3, u = 3, k = 0, 4x - 3k = 12, 2j+2 = 12 => j = 5, 2x - k = 6 which violates 2x - k <= u
//     Is there a solution when x = 3? Then we would need to have
//       0 <= 3 <= u < 2j+2 = 12 - 3k and 0 <= 6 - k <= u
//       Try k = 3 then 3 <= u < 2j+2 = 3 no
//       Try k = 2 then 3 <= u < 2j+2 = 6 and 0 <= 4 <= u
//       implies j = 2, and u can be 5
// CHECK:
//   mutually_recursive_a(3)
//   = mutually_recursive_b(6) - 1
//   = (mutually_recursive_a(4) - 2) - 1
//   = mutually_recursive_a(4) - 3
//   = (mutually_recursive_b(8) - 1) - 3
//   = mutually_recursive_b(8) - 4
//   = (mutually_recursive_a(6) - 2) - 4
//   = mutually_recursive_a(6) - 6
//   = 6 - 6
//   = 0
