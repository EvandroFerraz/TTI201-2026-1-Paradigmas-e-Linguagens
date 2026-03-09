#include <stdio.h>

int main(){
    int numero = 42; // Uma variável inteira
    // |numero: 42 / x0121415| - endereço de memória hipotético para guardar o 42
    // Uma variável (ponteiro) que guarda um endereço de memória 
    //  onde um inteiro será guardado
    int *ponteiro;
    ponteiro = &numero; // Atribuindo o endereço da variável número ao ponteiro
    // |ponteiro: x0121415|

    printf("Valor de 'numero: %d\n", numero); // 42, valor dentro da variável
    printf("Endereço de 'numero': %p\n", (void*)&numero); // x0121415, endereço do 42
    printf("Endereço de 'numero': %p\n", ponteiro); // x0121415, endereço do 42
    printf("Valor apontado por porteiro: %d", *ponteiro); // 42, valor dentro do endereço

    *ponteiro = 100;
    //|ponteiro: x0121415| |*ponteiro: 42|, 42 é substituido pelo 100.
     printf("Valor apontado por porteiro: %d", *ponteiro); // 100, que substituiu o 42
}