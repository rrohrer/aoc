#include <stdio.h>

int main(int argc, char **argv) {
  FILE *ifp = fopen("input.txt", "rw");
  if (ifp == NULL) {
    printf("Could not open input file\n");
    return 0;
  }

  return 0;
}
