int basic(int a, int b) {
  return a + b - 3;
}

int basic_binops(int a, int b) {
  int c = a + b - (77 * a) + 1;
  int d = (c & 23) / (a | 99);
  int e = (d ^ a) % (c << 3);
  return e >> d;
}

int basic_if(int a, int b) {
  if (a < b) {
    return a - b;
  } else {
    return (a-1) * (b-1);
  }
}
