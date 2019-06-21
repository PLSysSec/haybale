int no_args() {
  return 0;
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

int conditional(int a, int b) {
  if (a > b) {
    return a - b;
  } else {
    return (a-1) * (b-1);
  }
}
