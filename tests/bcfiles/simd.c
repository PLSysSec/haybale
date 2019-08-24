#include <stdint.h>
#include <stdbool.h>

__attribute__((noinline)) uint32_t callee(uint32_t* x, uint32_t* y, uint32_t*__restrict z) {
  for(int i = 0; i < 16; i++) {
    x[i] = i;
    y[i] = i + 2;
  }
  for (int i = 0; i < 16; i++) z[i] = x[i] + y[i];
  uint32_t sum = 0;
  for (int i = 0; i < 16; i++) sum += z[i];
  return sum;
}

uint32_t simd_add_autovectorized() {
  // actually allocate the arrays so that the symex gives them concrete addresses
  uint32_t x[16];
  uint32_t y[16];
  uint32_t z[16];
  return callee(x, y, z);
}
