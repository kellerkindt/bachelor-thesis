#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct Computer {
  char* model;
} Computer;

Computer* erstelle_computer(char* model) {
    Computer* computer = malloc(sizeof(Computer));
    computer->model = malloc(strlen(model)+1);
    strcpy(computer->model, model);
    return computer;
}

Computer* klone_computer(Computer* original) {
    Computer* klon = malloc(sizeof(Computer));
    klon->model = original->model;
    return klon;
}

void loesche_computer(Computer* computer) {
    free(computer->model);
    free(computer);
}

void main() {
    Computer* c1 = erstelle_computer("Lenovo");
    Computer* c2 = klone_computer(c1);
    printf("Model Nr1=%s, Nr2=%s\n", c1->model, c2->model);
    loesche_computer(c1);
    printf("Model Nr2=%s\n", c2->model);
}

