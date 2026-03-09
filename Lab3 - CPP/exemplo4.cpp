/*EXERCÍCIO 4 – Escreva um programa em C++ que solicite ao usuário uma temperatura em 
graus Celsius e a converta para Fahrenheit e Kelvin.*/

#include <iostream>
#include <iomanip> // Para formatar a saída

using namespace std;

int main() {
    double celsius, fahrenheit, kelvin;

    // Entrada do usuário
    cout << "Digite a temperatura em graus Celsius: ";
    cin >> celsius;

    // Conversão para Fahrenheit e Kelvin
    fahrenheit = (celsius * 9.0 / 5.0) + 32;
    kelvin = celsius + 273.15;

    // Exibição dos resultados formatados
    cout << fixed << setprecision(2);
    cout << "Em Fahrenheit: " << fahrenheit << endl;
    cout << "Em Kelvin: " << kelvin << endl;   
}