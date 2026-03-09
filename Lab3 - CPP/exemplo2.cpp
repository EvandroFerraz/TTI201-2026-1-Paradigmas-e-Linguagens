/* EXERCÍCIO 2 - Escreva um programa em C++ que solicite ao usuário um número inteiro 
positivo e verifique se ele é um número primo. Um número é considerado primo se for maior 
que 1 e divisível apenas por 1 e por ele mesmo. O programa deve exibir uma mensagem 
informando se o número é primo ou não.*/

#include <iostream>

using namespace std;

int main() {
    int num;
    bool ehPrimo = true;

    // Entrada do número
    cout << "Digite um numero inteiro positivo: ";
    cin >> num;

    // Verificação de número primo
    if (num <= 1) {
        ehPrimo = false;
    } else { // num > 1
        // num = 4
        // i = 2; i < 4; i++
        // if(4 % 2 == 0)
        for (int i = 2; i < num; i++) {
            if (num % i == 0) {
                ehPrimo = false;
                break;
            }
        }
    }

    // Exibição do resultado
    if (ehPrimo) {
        cout << num << " e um numero primo." << endl;
    } else {
        cout << num << " nao e um numero primo." << endl;
    }
}