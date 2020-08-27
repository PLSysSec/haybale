#include <stdlib.h>

struct TwoInts {
  int el1;
  int el2;
};

struct WithPointer {
  volatile struct TwoInts ti;
  volatile struct TwoInts* volatile* ti_2;
};

// function that takes a pointer to a struct as an argument
// really the point here is to get a GEP with multiple indices on -O3
__attribute__((noinline))
int called(struct WithPointer* wp, int x) {
  wp->ti.el2 = x - 3;
  return wp->ti_2[0]->el2;
}

int with_ptr(int x) {
  volatile struct TwoInts* ti = (volatile struct TwoInts*) malloc(sizeof(struct TwoInts));
  struct WithPointer* wp = (struct WithPointer*) malloc(sizeof(struct WithPointer));
  if (wp == NULL || ti == NULL) {
    return -1;
  }
  wp->ti.el2 = 0;
  wp->ti_2 = &ti;
  wp->ti_2[0] = &wp->ti;
  return called(wp, x);
}
