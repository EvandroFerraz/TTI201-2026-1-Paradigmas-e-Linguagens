#include <stdio.h>

int main(){
    int i, j;
    printf("\nEntre com dois numeros inteiros: ");
    scanf("%d%d", &i, &j); // Se i = 5 e j = 7
    printf("\n%d == %d e %d\n", i, j, i==j); // 5 == 7, 0
    printf("\n%d != %d e %d\n", i, j, i!=j); // 5 != 7, 1
    printf("\n%d <= %d e %d\n", i, j, i<=j); // 5 <= 7, 1
    printf("\n%d >= %d e %d\n", i, j, i>=j); // 5 >= 7, 0
    printf("\n%d < %d e %d\n", i, j, i<j); // 5 < 7, 1
    printf("\n%d > %d e %d\n", i, j, i>j); // 5 > 7, 0

    printf("informe dois números(cada um sendo 0 ou 1): ");
    scanf("%d%d", &i, &j); // Se i = 1 e j = 0
    printf("%d AND %d é %d\n", i, j, i && j); // 1 && 0 = 0
    printf("%d OR %d é %d\n", i, j, i || j); // 1 || 0 = 1
    printf("NOT %d é %d\n", i, !i); // !i = !1 = 0
}