int basic(int a, int b) {
  return a + b - 3;
}

int basic_if(int a, int b) {
  if (a < b) {
    return a - b;
  } else {
    return (a-1) * (b-1);
  }
}
