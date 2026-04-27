// ============================================================
// Exercício 3 — Cálculo de Fatorial
// O fatorial de n (escrito n!) é o produto de todos os
// inteiros positivos de 1 até n.
// Fórmula: n! = n × (n-1) × (n-2) × ... × 1
// Exemplos:
//   5! = 5 × 4 × 3 × 2 × 1 = 120
//   3! = 3 × 2 × 1 = 6
//   0! = 1 (por definição matemática)
// ============================================================

// Importa o módulo de entrada/saída para ler do teclado.
use std::io;

// ------------------------------------------------------------
// Função: fatorial
// Recebe um número sem sinal de 64 bits (u64) e retorna
// o seu fatorial, também como u64.
//
// Por que u64 e não u32?
// Fatoriais crescem muito rápido. O u32 suporta até ~4,2 bilhões,
// mas 13! já passa disso (6,2 bilhões). O u64 suporta até
// ~18,4 quintilhões, permitindo calcular até 20! com segurança.
// ------------------------------------------------------------
fn fatorial(n: u64) -> u64 {
    // Começa com 1 porque:
    // - 0! = 1 por definição (o loop não executa para n=0)
    // - Multiplicar por 1 não altera o resultado
    let mut resultado = 1;

    // Itera de 2 até n (inclusive), acumulando o produto
    // O ..= cria um range inclusivo: para n=5, percorre 2, 3, 4, 5
    // Começa em 2 pois multiplicar por 1 seria desnecessário
    for i in 2..=n {
        resultado *= i; // Equivale a: resultado = resultado * i
    }

    // Retorna o produto acumulado (última expressão sem ; = retorno)
    resultado
}

// ------------------------------------------------------------
// Função principal: lê um número do usuário e exibe o fatorial
// ------------------------------------------------------------
fn main() {
    println!("Digite um número inteiro positivo:");

    // Cria uma string vazia para armazenar o que o usuário digitar
    let mut entrada = String::new();

    // Lê uma linha do teclado e armazena em `entrada`
    // &mut passa referência mutável — exigido pelo borrow checker
    // expect define mensagem de erro caso a leitura falhe
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    // trim() remove espaços e a quebra de linha do final da string
    // parse() tenta converter para u64 (inteiro sem sinal de 64 bits)
    // Err(_) captura qualquer falha de conversão, descartando o detalhe
    let numero: u64 = match entrada.trim().parse() {
        Ok(n) => n,   // Conversão bem-sucedida: usa o número
        Err(_) => {   // Qualquer erro (texto, número negativo, etc.)
            println!("Por favor, digite um número inteiro positivo válido.");
            return;   // Encerra o programa sem continuar
        }
    };

    // Chama a função e exibe o resultado no formato "n! = resultado"
    // O {} é o placeholder que será substituído pelo valor da variável
    println!("{}! = {}", numero, fatorial(numero));
}
