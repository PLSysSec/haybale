#include <stdint.h>

int no_args_zero() {
  return 0;
}

int no_args_nozero() {
  return 1;
}

int one_arg(int a) {
  return a - 3;
}

int two_args(int a, int b) {
  return a + b - 3;
}

int three_args(int a, int b, int c) {
  return a + b + c - 3;
}

int four_args(int a, int b, int c, int d) {
  return a + b + c + d - 3;
}

int five_args(int a, int b, int c, int d, int e) {
  return a + b + c + d + e - 3;
}

int binops(int a, int b) {
  int c = a + b - (77 * a) + 1;
  int d = (c & 23) / (a | 99);
  int e = (d ^ a) % (c << 3);
  return e >> d;
}

// this function can only return zero in its true branch
int conditional_true(int a, int b) {
  if (a > b) {
    return (a-1) * (b-1);
  } else {
    return (a + b) % 3 + 10;
  }
}

// this function can only return zero in its false branch
int conditional_false(int a, int b) {
  if (a > b) {
    return (a + b) % 3 + 10;
  } else {
    return (a-1) * (b-1);
  }
}

int conditional_nozero(int a, int b) {
  if (a > 2) {
    return a;
  } else if (b <= 0) {
    return b - 3;
  } else if (a <= 0) {
    return a - 7;
  } else {
    return a * b;
  }
}

int conditional_with_and(int a, int b) {
  if (a > 3 && b > 4) {
    return 0;
  } else {
    return 1;
  }
}

int has_switch(int a, int b) {
  switch (a - b) {
    case 0: return -1;
    case 1: return 3;
    case 2: return a - 3;
    case 3: return a * b + 1;
    case 33: return -300;
    case 451: return -5;
    default: return a - b - 1;
  }
}

int8_t int8t(int8_t a, int8_t b) {
  return a + b - 3;
}

int16_t int16t(int16_t a, int16_t b) {
  return a + b - 3;
}

int32_t int32t(int32_t a, int32_t b) {
  return a + b - 3;
}

int64_t int64t(int64_t a, int64_t b) {
  return a + b - 3;
}

int64_t mixed_bitwidths(int8_t i8, int16_t i16, int32_t i32, int64_t i64) {
  return i8 + i16 + i32 + i64 - 3;
}
