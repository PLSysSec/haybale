int load_and_store(volatile int* ptr, int a) {
  *ptr = a - 3;
  return *ptr;
}

int local_ptr(int a) {
  volatile int stored;
  volatile int* ptr = &stored;
  *ptr = a - 3;
  return *ptr;
}

int overwrite(volatile int* ptr, int a) {
  *ptr = 0;
  *ptr = 2;
  *ptr = a - 3;
  return *ptr;
}

int load_and_store_mult(volatile int* ptr, int a) {
  *ptr = a;
  *ptr = *ptr + 10;
  *ptr = *ptr - 13;
  return *ptr;
}

int array(volatile int* ptr, int a) {
  ptr[10] = a - 3;
  return ptr[10];
}

int pointer_arith(volatile int* ptr, int a) {
  volatile int* temp_ptr = ptr;
  *temp_ptr = a - 0;
  temp_ptr++;
  *temp_ptr = a - 1;
  temp_ptr++;
  *temp_ptr = a - 2;
  temp_ptr++;
  *temp_ptr = a - 3;
  temp_ptr++;
  *temp_ptr = a - 4;
  temp_ptr++;
  *temp_ptr = a - 5;
  temp_ptr++;
  *temp_ptr = a - 6;
  temp_ptr++;

  return ptr[3];
}
