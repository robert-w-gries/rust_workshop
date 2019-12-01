#include <stdio.h>

int main() {
    int si = -1;
    int x;
    unsigned int ui = 1;
    x = si < (int) ui;
    if (x)
      printf("\n -1 < 1: true");
    else
      printf("\n -1 < 1: false");

    return 0;
}

