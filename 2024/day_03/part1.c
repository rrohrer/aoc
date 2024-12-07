#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

bool is_digit(char c) { return c >= '0' && c <= '9'; }

bool match(char **data, int *a, int *b, int len) {
  char intro[] = {'m', 'u', 'l', '('};
  char *ptr = *data;
  char *a_start;
  char *b_start;

  for (int i = 0; i < 4 && (ptr - *data) < len; ++i) {
    if (*ptr != intro[i]) {
      *data = i == 0 ? ptr + 1 : ptr;
      return false;
    }
    ++ptr;
  }

  a_start = ptr;
  for (int i = 0; i < 3 && (ptr - *data) < len; ++i) {
    if (!is_digit(*ptr) && i == 0) {
      *data = ptr;
      return false;
    }

    if (!is_digit(*ptr)) {
      break;
    }

    ++ptr;
  }

  if ((ptr - *data) >= len || *ptr != ',') {
    *data = ptr;
    return false;
  }
  ++ptr;

  b_start = ptr;
  for (int i = 0; i < 3 && (ptr - *data) < len; ++i) {
    if (!is_digit(*ptr) && i == 0) {
      *data = ptr;
      return false;
    }

    if (!is_digit(*ptr)) {
      break;
    }

    ++ptr;
  }

  if ((ptr - *data) >= len || *ptr != ')') {
    *data = ptr;
    return false;
  }
  ++ptr;
  *data = ptr;
  *a = atoi(a_start);
  *b = atoi(b_start);
  return true;
}

int main(int argc, char **argv) {
  FILE *ifp = fopen("input.txt", "rb");
  if (ifp == NULL) {
    printf("could not find input\n");
    return 0;
  }
  fseek(ifp, 0L, SEEK_END);
  int length = ftell(ifp);
  rewind(ifp);
  fseek(ifp, 0L, SEEK_SET);
  char *data = malloc(length + 1);
  int read_len = fread(data, sizeof(char), length, ifp);
  if (read_len != length) {
    printf("unable to read entire file\n");
    return 0;
  }
  fclose(ifp);
  data[length] = 0;

  char *end = data + length;
  char *iter = data;
  int a, b;
  long long sum = 0;
  while (iter < end) {
    if (match(&iter, &a, &b, length - (iter - data))) {
      sum += a * b;
      printf("(%d, %d) ", a, b);
    }
  }
  free(data);
  printf("sum: %lld\n", sum);
  return 0;
}
