#include <stdio.h>
#include <inttypes.h>

extern long entry();

int main(int argc, char **argv) {
  printf("%ld", entry());
  return 0;
}