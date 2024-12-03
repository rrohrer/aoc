

#include "input.h"

#define INPUT_SIZE 1000
#define INPUT_BASE 10162
#define INPUT_MAX 99987
#define HISTOGRAM_SIZE INPUT_MAX - INPUT_BASE + 1

int part2_fast() {
  int histogram[HISTOGRAM_SIZE] = {0};
  for (int i = 0; i < INPUT_SIZE; ++i) {
    ++histogram[right_list[i] - INPUT_BASE];
  }

  int sum = 0;
  for (int i = 0; i < INPUT_SIZE; ++i) {
    int current = left_list[i];
    sum += current * histogram[current - INPUT_BASE];
  }

  return sum;
}
