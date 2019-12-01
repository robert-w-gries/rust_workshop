#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
 char *var = malloc(10);
 strcpy(var,"hello");
 printf("\nvar before free %s", var);
 free(var);
 printf("\nvar after free %s", var); // use after free

 return 1; 
}
