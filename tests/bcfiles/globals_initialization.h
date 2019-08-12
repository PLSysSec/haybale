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
