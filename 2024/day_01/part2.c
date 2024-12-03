
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INPUT_SIZE 1000
#define INPUT_BASE 10162
#define INPUT_MAX 99987
#define HISTOGRAM_SIZE INPUT_MAX - INPUT_BASE + 1

int comp_int(void const *a, void const *b) {
  int *ai = (int *)a;
  int *bi = (int *)b;
  return *ai - *bi;
}

int main(int argc, char **argv) {
  (void)argc;
  (void)argv;

  FILE *fp = fopen("input_1.txt", "rt");
  if (fp == NULL) {
    printf("Could not open input file.\n");
    return 0;
  }

  char buffer[128];
  int left_list[INPUT_SIZE];
  int right_list[INPUT_SIZE];
  int index = 0;
  while (fgets(buffer, sizeof(buffer), fp)) {
    char *token = strtok(buffer, " ");
    left_list[index] = atoi(token);
    token = strtok(NULL, " ");
    right_list[index] = atoi(token);
    ++index;
  }
  fclose(fp);

  int histogram[HISTOGRAM_SIZE] = {0};
  for (int i = 0; i < INPUT_SIZE; ++i) {
    ++histogram[right_list[i] - INPUT_BASE];
  }

  int sum = 0;
  for (int i = 0; i < INPUT_SIZE; ++i) {
    int current = left_list[i];
    sum += current * histogram[current - INPUT_BASE];
  }

  printf("the similarity score of the list is: %d\n", sum);

  return 0;
}
