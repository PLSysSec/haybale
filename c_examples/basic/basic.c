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
  if (a < 2) {
    return a;
  } else if (b == 0) {
    return b + 3;
  } else {
    return a * b;
  }
}
