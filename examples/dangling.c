#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char* case1() {
   char x[10]; 
   strcpy(x,"hello");
   printf("\ncase 1 %s", x);
   return x;
}

char *case2() {
  char *x = (char*) malloc(10);
  strcpy(x,"hello");
  printf("\ncase 2 %s", x);
  free(x);
  return x;
}

int main() {
  printf("\nafter case1() %s", case1());
  printf("\nafter case2() %s", case2());
  return 0;
}
