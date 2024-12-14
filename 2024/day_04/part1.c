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

  // 8
  char xmas[] = {'X', 'M', 'A', 'S'};
  int indexes[][4][2] = {
      {{0, 0}, {1, 0}, {2, 0}, {3, 0}},
      {{0, 0}, {-1, 0}, {-2, 0}, {-3, 0}},
      {{0, 0}, {0, 1}, {0, 2}, {0, 3}},
      {{0, 0}, {0, -1}, {0, -2}, {0, -3}},
      {{0, 0}, {-1, -1}, {-2, -2}, {-3, -3}},
      {{0, 0}, {1, 1}, {2, 2}, {3, 3}},
      {{0, 0}, {-1, 1}, {-2, 2}, {-3, 3}},
      {{0, 0}, {1, -1}, {2, -2}, {3, -3}},
  };

  int sum = 0;
  for (int y = 0; y < 140; ++y) {
    for (int x = 0; x < 140; ++x) {
      for (int i = 0; i < 8; ++i) {
        int l_count = 0;
        for (int l = 0; l < 4; ++l) {
          char c = 0;
          if (!tap_grid(data, x + indexes[i][l][0], y + indexes[i][l][1], &c))
            break;
          if (c != xmas[l])
            break;
          ++l_count;
        }
        if (l_count == 4)
          ++sum;
      }
    }
  }

  printf("total number of XMAS: %d\n", sum);
  return 0;
}
