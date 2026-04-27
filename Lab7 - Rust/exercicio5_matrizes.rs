// ============================================================
// Exercício 5 — Soma de Matrizes 3×3
// Lê dois conjuntos de 9 números do usuário, organiza cada um
// em uma matriz 3×3 e exibe a soma elemento a elemento.
//
// Exemplo:
//   Matriz 1        Matriz 2        Resultado
//   1  2  3         9  8  7         10  10  10
//   4  5  6    +    6  5  4    =    10  10  10
//   7  8  9         3  2  1         10  10  10
// ============================================================

// Importa o módulo de entrada/saída para ler do teclado.
use std::io;

// ------------------------------------------------------------
// Função: soma_matrizes
// Recebe duas matrizes 3×3 de i32 por valor (cópia)
// e retorna uma nova matriz com a soma elemento a elemento.
//
// O tipo [[i32; 3]; 3] significa:
//   - Um array de 3 elementos...
//   - ...onde cada elemento é um array de 3 inteiros i32
// O tamanho faz parte do tipo em Rust — arrays são sempre fixos.
// ------------------------------------------------------------
fn soma_matrizes(m1: [[i32; 3]; 3], m2: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // Inicializa a matriz resultado com zeros em todas as posições
    let mut r = [[0; 3]; 3];

    // Loop externo: percorre as linhas (i = 0, 1, 2)
    for i in 0..3 {
        // Loop interno: percorre as colunas (j = 0, 1, 2)
        // 0..3 é range exclusivo — percorre 0, 1, 2 (não inclui 3)
        for j in 0..3 {
            // Soma os elementos na mesma posição das duas matrizes
            r[i][j] = m1[i][j] + m2[i][j];
        }
    }

    // Retorna a matriz resultado (sem ; = valor de retorno da função)
    r
}

// ------------------------------------------------------------
// Função: ler_matriz
// Solicita ao usuário os 9 elementos de uma matriz 3×3,
// lendo um por vez, e retorna a matriz preenchida.
// ------------------------------------------------------------
fn ler_matriz() -> [[i32; 3]; 3] {
    // Inicializa a matriz com zeros — será preenchida pelo usuário
    let mut m = [[0; 3]; 3];

    // Percorre cada posição da matriz linha por linha
    for i in 0..3 {
        for j in 0..3 {
            // Mostra a posição atual para orientar o usuário
            // i+1 e j+1 para exibir de 1 a 3 (mais intuitivo que 0 a 2)
            println!("Digite o elemento [{},{}]:", i + 1, j + 1);

            // Cria uma nova string vazia para cada leitura
            let mut e = String::new();

            io::stdin()
                .read_line(&mut e)
                .expect("Erro ao ler entrada");

            // unwrap_or(0) é alternativa compacta ao match:
            // se parse() funcionar, usa o número; se falhar, usa 0
            m[i][j] = e.trim().parse().unwrap_or(0);
        }
    }

    // Retorna a matriz completamente preenchida
    m
}

// ------------------------------------------------------------
// Função principal: lê as duas matrizes, soma e exibe o resultado
// ------------------------------------------------------------
fn main() {
    println!("=== Primeira matriz ===");
    let m1 = ler_matriz(); // Lê e armazena a primeira matriz

    println!("=== Segunda matriz ===");
    let m2 = ler_matriz(); // Lê e armazena a segunda matriz

    // Chama a função de soma e armazena a matriz resultante
    let r = soma_matrizes(m1, m2);

    println!("=== Resultado da soma ===");

    // O & empresta a matriz sem mover a posse (borrow)
    // Sem o &, a variável r seria consumida pelo loop e não poderia
    // ser usada depois — com & apenas emprestamos para leitura
    for linha in &r {
        // Itera sobre cada valor da linha atual
        for val in linha {
            // {:4} formata o número ocupando 4 caracteres de largura,
            // alinhando as colunas visualmente mesmo com números grandes
            print!("{:4}", val);
        }
        // println! sem argumentos apenas quebra a linha ao final de cada linha da matriz
        println!();
    }
}
