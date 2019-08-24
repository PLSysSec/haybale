// This file uses OpenCL to more easily generate LLVM first-class vector operations

uint simd_add(uint x, uint y) {
  uint4 a = (uint4)(x);
  uint4 b = { y, y + 1, y + 2, y + 3 };
  uint4 c = a + b;
  return c.x + c.y + c.z + c.w;
}

uint simd_ops(uint x, uint y) {
  uint4 a = (uint4)(x);
  uint4 b = { y, y + 1, y + 2, y + 3 };
  uint4 c = a + b - (uint4)(3);
  uint4 d = c * 17;
  uint4 e = (d & (~a)) | b;
  uint4 f = e >> 2;
  uint4 g = f << (uint4)(2,3,4,5);
  return g.x + g.y + g.z + g.w;
}

uint simd_select(uint x, uint y) {
  uint4 a = (uint4)(x);
  uint4 b = { y, y + 1, y + 2, y + 3 };
  uint4 c = a < b ? a : b;
  return c.x + c.y + c.z + c.w;
}

uint simd_typeconversions(uint x, uint y) {
  uint4 a = (uint4)(x);
  uint4 b = { y, y + 10, y + 3, y + 30 };
  ulong4 c = { a.x, a.y, a.z, a.w };
  ulong4 d = { b.x, b.y, b.z, b.w };
  ulong4 e = d - c;
  uint4 f = { e.x, e.y, e.z, e.w };
  return f.x + f.y + f.z + f.w;
}
