#include<stdlib.h>

int may_exit(int a) {
  if (a > 2) {
    exit(1);
  } else {
    return 1;
  }
}
