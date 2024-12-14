#include <stdbool.h>
#include <stdio.h>

bool tap_grid(char const *grid, int x, int y, char *c) {
  if (x >= 140 || x < 0 || y >= 140 || y < 0) {
    return false;
  }
  *c = *(grid + y * 141 + x);
  return true;
}

int main(int argc, char **argv) {
  FILE *ifp = fopen("input.txt", "rb");
  if (ifp == NULL) {
    printf("Could not find the input file\n");
    return 0;
  }

  char data[142 * 142] = {0};
  int row = 0;
  while (fgets(data + row * 141, sizeof(char) * 142, ifp)) {
    ++row;
  }
  fclose(ifp);

  int sum = 0;
  for (int y = 0; y < 140; ++y) {
    for (int x = 0; x < 140; ++x) {
      char c = 0;
      if (!tap_grid(data, x, y, &c) || c != 'A')
        continue;

      bool left = false;
      if (tap_grid(data, x - 1, y - 1, &c) && c == 'M' &&
          tap_grid(data, x + 1, y + 1, &c) && c == 'S')
        left = true;
      if (tap_grid(data, x - 1, y - 1, &c) && c == 'S' &&
          tap_grid(data, x + 1, y + 1, &c) && c == 'M')
        left = true;

      bool right = false;
      if (tap_grid(data, x - 1, y + 1, &c) && c == 'M' &&
          tap_grid(data, x + 1, y - 1, &c) && c == 'S')
        right = true;
      if (tap_grid(data, x - 1, y + 1, &c) && c == 'S' &&
          tap_grid(data, x + 1, y - 1, &c) && c == 'M')
        right = true;

      if (left && right)
        ++sum;
    }
  }

  printf("total number of XMAS: %d\n", sum);
  return 0;
}
