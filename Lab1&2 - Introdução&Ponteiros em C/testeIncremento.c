#include <stdio.h>

int main(){
    int x = 23;
    int y = x++; // pós-incremento, y = 23 e x = 24
    printf("O valor de x e %d e de y e %d\n", x, y);

    x = 23;
    y = ++x; // pré-incremento, x = 24 e y = 24
    printf("O valor de x e %d e de y e %d", x, y);
}