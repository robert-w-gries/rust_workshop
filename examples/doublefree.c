#include <stdlib.h>
#include <string.h>

int main() {
  int len = strlen("Hello world!");
  char* owner = (char *) malloc (len+1);
  char *new_owner = owner;
  strcpy(owner,("Hello world!"));
  free(owner);
  free(new_owner);
}

