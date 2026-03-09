/*EXERCÍCIO 3 – Escreva um programa em C++ que solicite ao usuário um número inteiro 
positivo e calcule o fatorial desse número. O fatorial de um número n (n!) é definido como 
o produto de todos os inteiros positivos de 1 até n.*/

#include <iostream>

using namespace std;

int main() {
    int num;
    unsigned long long fatorial = 1; // Variável para armazenar o resultado

    // Entrada do número
    cout << "Digite um numero inteiro positivo: ";
    cin >> num;

    // Validação de entrada
    if (num < 0) {
        cout << "O fatorial nao e definido para numeros negativos." << endl;
    } else {
        // Cálculo do fatorial
        for (int i = 1; i <= num; i++) {
            fatorial *= i;
        }

        // Exibição do resultado
        cout << "O fatorial de " << num << " e: " << fatorial << endl;
    }
}