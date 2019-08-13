struct SomeStruct {
  const int field1;
  const int field2;
  const int field3;
};

struct StructWithPointers {
  const int field1;
  const int* intptr;
  const struct SomeStruct* ssptr;
  const struct StructWithPointers* swpptr;
};

struct StructWithFunctionPointer {
  const int field1;
  int (*const funcptr)(int, int);
  void* voidfuncptr;
};

int foo();
int bar(int, int);
