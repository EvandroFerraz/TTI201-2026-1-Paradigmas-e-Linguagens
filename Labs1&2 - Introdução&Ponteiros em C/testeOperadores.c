// para compilar no terminal:
// gcc testeOperadores.c -o teste.exe
#include <stdio.h>

int main(){
    int a = 17, b = 3;
    int x, y;

    float z = 17., z1, z2;

    x = a / b; // Divisão inteira, x recebe um int
    printf("O resultado de x e: %d\n", x);

    y = a % b; // Resto de divisão
    printf("O resultado de y e: %d\n", y);

    z1 = z / b; // Divisão real, z1 recebe um float
    printf("O resultado de z1 e: %f\n", z1);

    z2 = a / b; // Divisão inteira, porém como z2 é float,
    // vai ser convertido em um float
    printf("O resultado de z2 e: %f\n", z2);
}