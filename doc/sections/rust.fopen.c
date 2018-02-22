#include <stdio.h>

void main(void) {
   FILE *file = fopen("private.key", "w");
   fputs("42", file);
}
