#include <stdio.h>
int  main() {

        char a = 240;
        unsigned char b = 240;
        printf(" a = %x\n b = %x\n", a,b);
        if (a == b)
                printf(" a = b\n");
        else
                printf(" a != b\n");
}
