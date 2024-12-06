#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef int array_t[16];

int main(int argc, char **argv) {
  FILE *ifp = fopen("input.txt", "rt");
  if (ifp == NULL) {
    printf("could not open input file\n");
    return 0;
  }

  char line_buffer[32] = {0};
  array_t data[1024] = {0};
  int input_size = 0;
  while (fgets(line_buffer, sizeof(line_buffer), ifp)) {
    char *token = strtok(line_buffer, " ");
    while (token) {
      int index = ++data[input_size][0];
      data[input_size][index] = atoi(token);
      token = strtok(NULL, " ");
    }
    ++input_size;
  }

  // rules:
  // 1: all up or all down
  // 2: range 1-3
  int safe_sum = 0;
  for (int i = 0; i < input_size; ++i) {
    bool sign = (data[i][2] - data[i][1]) > 0;
    bool safe = true;
    for (int j = 1; j < data[i][0]; ++j) {
      int index = j + 1;
      int last = j;
      int delta = data[i][index] - data[i][last];
      if (sign != (delta > 0)) {
        safe = false;
        break;
      }
      delta = abs(delta);
      if (delta > 3 || delta < 1) {
        safe = false;
        break;
      }
    }
    if (safe)
      ++safe_sum;
  }

  printf("safe: %d\n", safe_sum);
  return 0;
}
