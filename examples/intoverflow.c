#include <stdio.h>
#include <limits.h>
char* foo (int x) {
  if ((x+1) > x) return "true" ;
  else return "false"; 
}

int main ( void ) {
  printf("INT_MAX = %d", INT_MAX);
  printf("\n(INT_MAX +1) > INT_MAX : %s",foo(INT_MAX));
  return 0;
}
