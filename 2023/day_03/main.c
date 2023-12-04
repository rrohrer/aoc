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

int main(int argc, char **argv) {
  (void)argc;
  (void)argv;

  grid_t grid = new_grid("small_input.txt");

  printf("%s", grid.data);
  printf("width: %d, height: %d\n", grid.width, grid.height);

  free_grid(grid);
  return 0;
}
