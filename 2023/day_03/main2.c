
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
  char *data;
  int width;
  int height;
} grid_t;

grid_t new_grid(char const *path) {
  FILE *file = fopen(path, "rt");
  fseek(file, 0, SEEK_END);
  int size = ftell(file);
  fseek(file, 0, SEEK_SET);

  char *data = malloc(size + 1);
  fread(data, size, 1, file);
  fclose(file);
  data[size] = 0;

  int width = 0;
  int height = 0;
  for (int i = 0; i < size; ++i) {
    if (data[i] == '\n') {
      if (width == 0) {
        width = i;
      }
      ++height;
    }
  }

  return (grid_t){.data = data, .width = width, .height = height};
}

void free_grid(grid_t g) { free(g.data); }

char *g_index(grid_t *g, int x, int y) {
  if (x < 0 || y < 0 || x >= g->width || y >= g->height)
    return NULL;
  int offset = (g->width + 1) * y + x;
  return g->data + offset;
}

bool is_digit(char c) { return c >= '0' && c <= '9'; }

int process_number(grid_t *g, int *count, int x, int y) {
  char *c = g_index(g, x, y);
  if (c == NULL || *c == '.' || !is_digit(*c))
    return 1;

  *count += 1;

  while (c != NULL && is_digit(*c))
    c = g_index(g, ++x, y);

  c = g_index(g, --x, y);

  int sum = 0;
  int power = 1;
  while (c != NULL && is_digit(*c)) {
    sum += (*c - '0') * power;
    power *= 10;
    *c = '.';
    c = g_index(g, --x, y);
  }

  return sum;
}

int process_location(grid_t *g, int x, int y) {
  char *c = g_index(g, x, y);
  if (c != NULL && *c == '*') {
    int count = 0;
    int product = 1;
    product *= process_number(g, &count, x, y + 1);
    product *= process_number(g, &count, x + 1, y + 1);
    product *= process_number(g, &count, x + 1, y);
    product *= process_number(g, &count, x + 1, y - 1);
    product *= process_number(g, &count, x, y - 1);
    product *= process_number(g, &count, x - 1, y - 1);
    product *= process_number(g, &count, x - 1, y);
    product *= process_number(g, &count, x - 1, y + 1);
    return count == 2 ? product : 0;
  } else {
    return 0;
  }
}

int main(int argc, char **argv) {
  (void)argc;
  (void)argv;

  grid_t grid = new_grid("input.txt");

  printf("width: %d, height: %d\n", grid.width, grid.height);

  int ratio = 0;
  for (int y = 0; y < grid.height; ++y) {
    for (int x = 0; x < grid.width; ++x) {
      ratio += process_location(&grid, x, y);
    }
  }

  printf("Ratio: %d\n", ratio);

  free_grid(grid);
  return 0;
}
