/*EXERCÍCIO 5 – Escreva um programa em C++ que solicite ao usuário uma frase e conte o número de vogais 
(a, e, i, o, u) presentes na string. O programa deve considerar tanto maiúsculas quanto minúsculas e exibir 
o total de vogais encontradas.*/

#include <iostream>
#include <string> // Para manipulação de strings

using namespace std;

int main() {
    string frase;
    int contadorVogais = 0;

    // Entrada da frase
    cout << "Digite uma frase: ";
    getline(cin, frase); // Lê a linha inteira, incluindo espaços

    // Percorre a string para contar vogais
    for (char c : frase) {
        c = tolower(c); // Converte para minúscula para facilitar a verificação
        if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
            contadorVogais++;
        }
    }

    // Exibição do resultado
    cout << "Numero de vogais na frase: " << contadorVogais << endl;
}
