/*EXERCÍCIO 6 – Problema da Mochila (Knapsack 0/1) usando Programação Dinâmica*/

#include <iostream>
#include <vector>
#include <algorithm> // permitir o uso da função max()
using namespace std;

// Função para resolver o problema da Mochila 0/1 usando Programação Dinâmica
int knapsack(int capacidade, vector<int>& pesos, vector<int>& valores, int N) {
    
    // Criamos uma matriz dp[N+1][capacidade+1] inicializada com 0
    vector<vector<int>> dp(N + 1, vector<int>(capacidade + 1, 0));

    /* =========================================================================
       TESTE DE MESA:
       N = 3, capacidade = 5
       pesos = [2, 3, 4]
       valores = [4, 5, 6]
    
       // N+1 = linhas (4); capacidade+1 = colunas (6)
       A matriz 'dp' inicia assim (tudo 0):
       [0, 0, 0, 0, 0, 0] // Linha 0 (0 produtos)
       [0, 0, 0, 0, 0, 0] // Linha 1 (Até produto 1)
       [0, 0, 0, 0, 0, 0] // Linha 2 (Até produto 2)
       [0, 0, 0, 0, 0, 0] // Linha 3 (Até produto 3)
       ========================================================================= */

    for (int i = 1; i <= N; i++) {
        for (int w = 1; w <= capacidade; w++) {
            
            /* ITERANDO i = 1 (PRODUTO 1: peso=2, valor=4)
            - w=1: 2 > 1. Não cabe. dp[1][1] = dp[0][1] = 0
            - w=2: 2 <= 2. Cabe! max(4 + dp[0][2-2], dp[0][2]) = max(4+0, 0) = 4
            - w=3: 2 <= 3. Cabe! max(4 + dp[0][3-2], dp[0][3]) = max(4+0, 0) = 4
            - w=4: 2 <= 4. Cabe! max(4 + dp[0][4-2], dp[0][4]) = max(4+0, 0) = 4
            - w=5: 2 <= 5. Cabe! max(4 + dp[0][5-2], dp[0][5]) = max(4+0, 0) = 4
            Resultado Linha 1: [0, 0, 4, 4, 4, 4]
            Matriz após iteração:
                [0, 0, 0, 0, 0, 0] // Linha 0 intocada
                [0, 0, 4, 4, 4, 4] // Linha 1 atualizada (pega o produto 1 a partir da coluna 2, pois peso = 2)
                [0, 0, 0, 0, 0, 0] 
                [0, 0, 0, 0, 0, 0]
            */

            /* ITERANDO i = 2 (PRODUTO 2: peso=3, valor=5)
            - w=1: 3 > 1. Não cabe. dp[2][1] = dp[1][1] = 0
            - w=2: 3 > 2. Não cabe. dp[2][2] = dp[1][2] = 4
            - w=3: 3 <= 3. Cabe! max(5 + dp[1][3-3], dp[1][3]) = max(5+0, 4) = 5
            - w=4: 3 <= 4. Cabe! max(5 + dp[1][4-3], dp[1][4]) = max(5+0, 4) = 5
            - w=5: 3 <= 5. Cabe! max(5 + dp[1][5-3], dp[1][5]) = max(5+4, 4) = 9 (Pega Prod 1 e 2!)
            Resultado Linha 2: [0, 0, 4, 5, 5, 9]
            Matriz após iteração:
                [0, 0, 0, 0, 0, 0] 
                [0, 0, 4, 4, 4, 4] 
                [0, 0, 4, 5, 5, 9] // Linha 2 atualizada; Insere o produto 2 na coluna w = 5
                [0, 0, 0, 0, 0, 0]
                // A 5ª coluna (w = 5) representa capacidade = 5, ou seja, neste ponto é possível
                // inserir os produtos 1 e 2 pois a soma de seus pesos é menor ou igual a capacidade (5 <= 5)
            */

            /* ITERANDO i = 3 (PRODUTO 3: peso=4, valor=6)
            - w=1: 4 > 1. Não cabe. dp[3][1] = dp[2][1] = 0
            - w=2: 4 > 2. Não cabe. dp[3][2] = dp[2][2] = 4
            - w=3: 4 > 3. Não cabe. dp[3][3] = dp[2][3] = 5
            - w=4: 4 <= 4. Cabe! max(6 + dp[2][4-4], dp[2][4]) = max(6+0, 5) = 6
            - w=5: 4 <= 4. Cabe! max(6 + dp[2][5-4], dp[2][5]) = max(6+0, 9) = 9
            Resultado Linha 3: [0, 0, 4, 5, 6, 9]
            Matriz após iteração:
                [0, 0, 0, 0, 0, 0] 
                [0, 0, 4, 4, 4, 4] 
                [0, 0, 4, 5, 5, 9] 
                [0, 0, 4, 5, 6, 9] Considerando a capacidade máxima mostrada na 5ª coluna (w = 5), o maior
                // valor que podemos incluir ainda é 9 (produto 1 + produto 2, que somam peso = 2 + 3 = 5)
            */

            if (pesos[i - 1] <= w) {
                // Escolhemos entre pegar ou não o item atual
                dp[i][w] = max(valores[i - 1] + dp[i - 1][w - pesos[i - 1]], dp[i - 1][w]);
            } else {
                // Se o item não cabe, mantemos o valor anterior
                dp[i][w] = dp[i - 1][w];
            }
        }
    }
    
    // O valor final na última linha e última coluna será 9
    return dp[N][capacidade];
}

int main() {
    int N, capacidade;

    // Entrada do número de produtos e capacidade máxima
    cout << "Digite o numero de produtos: ";
    cin >> N;
    
    cout << "Digite a capacidade maxima do container: ";
    cin >> capacidade;

    vector<int> pesos(N), valores(N);

    // Entrada dos pesos dos produtos
    cout << "Digite os pesos dos produtos: ";
    for (int i = 0; i < N; i++) {
        cin >> pesos[i];
    }

    // Entrada dos valores dos produtos
    cout << "Digite os valores dos produtos: ";
    for (int i = 0; i < N; i++) {
        cin >> valores[i];
    }

    // Chamada da função Knapsack
    int resultado = knapsack(capacidade, pesos, valores, N);
    
    // Exibição do resultado
    cout << "O maior valor possivel que pode ser carregado e: " << resultado << endl;
}