#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INPUT_SIZE 1000

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

  qsort(left_list, INPUT_SIZE, sizeof(int), comp_int);
  qsort(right_list, INPUT_SIZE, sizeof(int), comp_int);

  int sum = 0;
  for (int i = 0; i < INPUT_SIZE; ++i) {
    sum += abs(left_list[i] - right_list[i]);
  }

  printf("the sum of the differences of the list is: %d\n", sum);

  return 0;
}
