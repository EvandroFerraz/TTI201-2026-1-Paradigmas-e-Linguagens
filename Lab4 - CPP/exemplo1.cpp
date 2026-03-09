/* EXERCÍCIO 1 - Um professor deseja calcular a média ponderada de um aluno com base em 
três notas e seus respectivos pesos. O programa deve solicitar as três notas e os três 
pesos correspondentes, calcular a média ponderada e exibir o resultado formatado com duas 
casas decimais.*/

#include <iostream>
#include <iomanip> // Para formatar a saída

using namespace std;

int main() {
    double nota1, nota2, nota3;
    double peso1, peso2, peso3;
    
    // Entrada de dados
    cout << "Digite a primeira nota e seu peso: ";
    cin >> nota1 >> peso1;
    
    cout << "Digite a segunda nota e seu peso: ";
    cin >> nota2 >> peso2;
    
    cout << "Digite a terceira nota e seu peso: ";
    cin >> nota3 >> peso3;
    
    // Cálculo da média ponderada
    double media = ((nota1 * peso1) + (nota2 * peso2) + (nota3 * peso3)) / (peso1 + peso2 + peso3);
    
    // Exibição do resultado formatado
    cout << fixed << setprecision(2);
    cout << "A media ponderada do aluno e: " << media << endl;
}